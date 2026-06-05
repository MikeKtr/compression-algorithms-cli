use std::{collections::HashMap, io::{BufReader, BufWriter, Read, Write}};
use crate::algorithms::traits::{CompressionAlgorithm,ReadSeek};
use std::io::{Seek,SeekFrom};
use crate::algorithms::tree::*;
struct pngCompression;



impl CompressionAlgorithm for pngCompression{
	fn compress(&self,source: &mut dyn ReadSeek, destination: &mut dyn Write) ->  std::io::Result<()> {
		let mut reader = BufReader::new(source);
		let mut writer = BufWriter::new(destination);

		let mut file_header = [0u8; 40];
		let mut img_header = [0u8; 40];

		reader.read_exact(&mut file_header);
		reader.read_exact(&mut img_header);

		let file_offset :i32 = i32::from_le_bytes(file_header[10..14].try_into().unwrap());
		let img_width = i32::from_le_bytes(img_header[4..8].try_into().unwrap());
		let img_length = i32::from_le_bytes(img_header[8..12].try_into().unwrap());

		

		reader.seek(SeekFrom::Start(file_offset as u64))?;





		
		let line_iterator = FilterReader::new(&mut reader,img_width as u32,img_length as u32);


		let mut counted_bytes = [0u64; 256];

		for line_result in line_iterator{
			let filtered_line = line_result?;

			for byte in filtered_line{
				counted_bytes[byte as usize] +=1;
			}
		}

		let mut map = HashMap::new();
		create_tree(&mut counted_bytes,&mut map);

		let file_len: u64 = counted_bytes.iter().sum();
		let file_len_bytes = file_len.to_le_bytes();
		writer.write_all(&file_len_bytes)?;

		let char_count = map.len();
		writer.write_all(&[char_count as u8])?;
		for i in 0..256{
			if counted_bytes[i] > 0{
				writer.write_all(&[i as u8])?;
				writer.write_all(&(counted_bytes[i].to_le_bytes()))?;
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
        return Ok(());

	}

	fn decompress(&self, source: &mut dyn ReadSeek, destination: &mut dyn Write) -> std::io::Result<()> {
		todo!()
	}

	fn name(&self) -> &'static str {
		return "Png Compression";
	}
}


pub struct FilterReader<'a,R: Read>{
	reader: &'a mut BufReader<R>,
    img_width: usize,
    img_length: usize,
    padding: usize,
    current_row: usize,
    prev_line: Vec<u8>,
}

impl<'a,R: Read> FilterReader<'a, R>{
	pub fn new(reader:&'a mut  BufReader<R>, width: u32, height: u32) -> Self{
		let bytes_per_row = (width * 3) as usize;
        let padding = (4 - (bytes_per_row % 4)) % 4;

		Self { reader, 
				img_width: bytes_per_row, 
				img_length: height as usize, 
				padding, current_row: 0, prev_line: vec![0u8;bytes_per_row] }
	}
}

impl<'a, R: Read> Iterator for FilterReader<'a, R> {
	type Item = std::io::Result<Vec<u8>>;

fn next(&mut self) -> Option<Self::Item> {
        if self.current_row >= self.img_length {
            return None;
        }

        let mut current_line = vec![0u8; self.img_width];

        if let Err(e) = self.reader.read_exact(&mut current_line) {
            return Some(Err(e));
        }

        if self.padding > 0 {
            let mut padding_buf = vec![0u8; self.padding];
            if let Err(e) = self.reader.read_exact(&mut padding_buf) {
                return Some(Err(e));
            }
        }

        let mut filtered_line = vec![0u8; self.img_width + 1];
        filtered_line[0] = 3;


            for i in 0..(self.img_width * 3) as usize {
                let left = if i >= 3 { current_line[i - 3] } else { 0 };
                let up = self.prev_line[i]; 

                let prediction = ((left as u16 + up as u16) / 2) as u8;

                filtered_line[i + 1] = current_line[i].wrapping_sub(prediction);
            }

			self.prev_line.copy_from_slice(&current_line);

		

       
        self.prev_line.copy_from_slice(&current_line);
        self.current_row += 1;

        Some(Ok(filtered_line))
    }
}