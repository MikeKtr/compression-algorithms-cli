use crate::algorithms::traits::{CompressionAlgorithm, ReadSeek};
use crate::algorithms::tree::{create_q, create_tree, TreeNode};
use crate::image::filter_reader::FilterReader;
use std::io::{Seek, SeekFrom};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    io::{BufReader, BufWriter, Read, Write},
};

pub struct PngCompression;

impl CompressionAlgorithm for PngCompression {
    fn compress(
        &self,
        source: &mut dyn ReadSeek,
        destination: &mut dyn Write,
    ) -> std::io::Result<()> {
        let mut reader = BufReader::new(source);
        let mut writer = BufWriter::new(destination);

        let mut file_header = [0u8; 14];
        let mut img_header = [0u8; 40];
        reader.read_exact(&mut file_header)?;
        reader.read_exact(&mut img_header)?;

        let file_offset = i32::from_le_bytes(file_header[10..14].try_into().unwrap());
        let img_width = i32::from_le_bytes(img_header[4..8].try_into().unwrap());
        let img_length = i32::from_le_bytes(img_header[8..12].try_into().unwrap());

        reader.seek(SeekFrom::Start(file_offset as u64))?;

        let line_iterator = FilterReader::new(&mut reader, img_width as u32, img_length as u32);

        let mut counted_bytes = [0u64; 256];
        for line_result in line_iterator {
            let filtered_line = line_result?;
            for byte in filtered_line {
                counted_bytes[byte as usize] += 1;
            }
        }

        let mut map = HashMap::new();
        create_tree(&mut counted_bytes, &mut map);

        let file_len: u64 = counted_bytes.iter().sum();
        writer.write_all(&file_len.to_le_bytes())?;
        let char_count = map.len();
        writer.write_all(&[char_count as u8])?;
        writer.write_all(&img_length.to_le_bytes())?;
        writer.write_all(&img_width.to_le_bytes())?;
        writer.write_all(&file_header)?;
        writer.write_all(&img_header)?;

        for i in 0..256 {
            if counted_bytes[i] > 0 {
                writer.write_all(&[i as u8])?;
                writer.write_all(&counted_bytes[i].to_le_bytes())?;
            }
        }

        let mut bufor: u8 = 0;
        let mut space: u8 = 8;

        reader.seek(SeekFrom::Start(file_offset as u64))?;
        let line_encoder = FilterReader::new(&mut reader, img_width as u32, img_length as u32);

        for line_result in line_encoder {
            let filtered_line = line_result?;
            for current_byte in filtered_line {
                let code = map.get(&current_byte).unwrap();
                let bits = code.bits;
                let len = code.len;
                for i in (0..len).rev() {
                    let bit = (bits >> i) & 1;
                    bufor = (bufor << 1) | (bit as u8);
                    space -= 1;
                    if space == 0 {
                        writer.write_all(&[bufor])?;
                        bufor = 0;
                        space = 8;
                    }
                }
            }
        }

        if space < 8 {
            bufor = bufor << space;
            writer.write_all(&[bufor])?;
        }
        writer.flush()?;
        Ok(())
    }

    fn decompress(
        &self,
        source: &mut dyn ReadSeek,
        destination: &mut dyn Write,
    ) -> std::io::Result<()> {
        let mut reader = BufReader::new(source);
        let mut writer = BufWriter::new(destination);

        let mut org_file_size_buf = [0; 8];
        reader.read_exact(&mut org_file_size_buf)?;
        let org_file_size = u64::from_le_bytes(org_file_size_buf);

        let mut org_sign_count_buf = [0; 1];
        reader.read_exact(&mut org_sign_count_buf)?;
        let org_sign_count = org_sign_count_buf[0];
        let loops = if org_sign_count == 0 {
            256
        } else {
            org_sign_count as usize
        };

        let mut org_img_length_buf = [0; 4];
        reader.read_exact(&mut org_img_length_buf)?;
        let org_img_length = u32::from_le_bytes(org_img_length_buf);

        let mut org_img_width_buf = [0; 4];
        reader.read_exact(&mut org_img_width_buf)?;
        let org_img_width = u32::from_le_bytes(org_img_width_buf);

        let mut file_hdr = [0u8; 14];
        let mut info_hdr = [0u8; 40];
        reader.read_exact(&mut file_hdr)?;
        reader.read_exact(&mut info_hdr)?;
        writer.write_all(&file_hdr)?;
        writer.write_all(&info_hdr)?;

        let mut pq = BinaryHeap::new();

        for _ in 0..loops {
            let mut sign_buf = [0; 1];
            let mut freq_buf = [0; 8];
            reader.read_exact(&mut sign_buf)?;
            reader.read_exact(&mut freq_buf)?;
            let sign = sign_buf[0];
            let freq = u64::from_le_bytes(freq_buf);
            let leaf = TreeNode::Leaf { byte: sign, freq };
            pq.push((Reverse(freq), Box::new(leaf)));
        }

        let (Reverse(_total_freq), root_node) = create_q(&mut pq);

        let mut current_node: &TreeNode = &*root_node;
        let mut decoded_bytes = 0;

        let mut prev_reconstructed_line = vec![0u8; (org_img_width * 3) as usize];
        let mut raw_line = vec![0u8; ((org_img_width * 3) + 1) as usize];
        let mut curr_index = 0;

        'outer: for byte_read in reader.bytes() {
            let byte = byte_read?;
            for n in (0..8).rev() {
                if decoded_bytes == org_file_size {
                    break 'outer;
                }
                let current_bit = ((byte >> n) & 1) == 1;
                match current_node {
                    TreeNode::Root { left, right, .. } => {
                        if current_bit {
                            current_node = &*right;
                        } else {
                            current_node = &*left;
                        }
                    }
                    TreeNode::Leaf { .. } => {}
                }
                if let TreeNode::Leaf { byte: sign, .. } = current_node {
                    raw_line[curr_index] = *sign;
                    curr_index += 1;
                    decoded_bytes += 1;
                    current_node = &*root_node;

                    if curr_index == raw_line.len() {
                        let mut bmp_line = vec![0u8; org_img_width as usize * 3];
                        for i in 0..(org_img_width as usize * 3) {
                            let filtered_byte = raw_line[i + 1];
                            let left = if i >= 3 { bmp_line[i - 3] } else { 0 };
                            let up = prev_reconstructed_line[i];
                            let prediction = ((left as u16 + up as u16) / 2) as u8;
                            bmp_line[i] = filtered_byte.wrapping_add(prediction);
                        }
                        writer.write_all(&bmp_line)?;

                        let padding_size = (4 - ((org_img_width as usize * 3) % 4)) % 4;
                        if padding_size > 0 {
                            writer.write_all(&vec![0u8; padding_size])?;
                        }

                        prev_reconstructed_line = bmp_line;
                        curr_index = 0;
                    }
                }
            }
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Png Compression"
    }
}
