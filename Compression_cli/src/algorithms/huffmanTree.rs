use std::{cmp::Reverse, collections::{BinaryHeap}, io::{BufReader, BufWriter, Read, Write}};
use crate::algorithms::traits::{CompressionAlgorithm,ReadSeek};
use std::collections::HashMap;
use std::io::{Seek,SeekFrom};


#[derive(Eq, Ord, PartialEq, PartialOrd,Debug)]
pub enum TreeNode{
	Leaf{
		byte: u8,
		freq: u64
	},
	Root{
		freq: u64,
		left: Box<TreeNode>,
		right: Box<TreeNode>
	}
}

#[derive(Clone, Copy, Debug)]
pub struct BitCode {
    pub bits: u32,   
    pub len: u8,
}
// Struktura nagłówka
// [ 8 bajtów: Łączna liczba znaków w pliku ]
// [ 1 bajt:   Liczba unikalnych znaków w słowniku ]
// [ N * 9 bajtów: Pary [1 bajt surowego znaku + 8 bajtów jego częstotliwości] ]

pub struct HuffmanCompression;

impl CompressionAlgorithm for HuffmanCompression{
	fn compress(&self,source: &mut dyn ReadSeek, destination: &mut dyn std::io::prelude::Write) ->  std::io::Result<()> {
		let mut reader = BufReader::new(source);
		let mut writer = BufWriter::new(destination);

		
		let mut counted_bytes  = [0u64; 256];
		for byte_read in reader.by_ref().bytes(){
			let byte = byte_read?;
			counted_bytes[byte as usize] += 1;
		}
		

		let mut pq = BinaryHeap::new();
		for i in 0..256{
			if counted_bytes[i] > 0{
			let leaf = TreeNode::Leaf { byte: i as u8, freq: counted_bytes[i] };
			pq.push((Reverse(counted_bytes[i]), Box::new(leaf)));
		}}

		while pq.len() > 1 {
			let (Reverse(freq1),left_node) = pq.pop().unwrap();
			let (Reverse(freq2),right_node) = pq.pop().unwrap();
			
			let combined_freq = freq1 + freq2;
			let parent = TreeNode::Root {
				freq: combined_freq,
				left: left_node, 
				right: right_node,
			};	
			pq.push((Reverse(combined_freq) ,Box::new(parent)));
		}
		let (Reverse(_total_freq), root_node) = pq.pop().unwrap();

		let mut map = HashMap::new();
		let initial_code = BitCode { bits: 0, len: 0 };

		traverse(&root_node, initial_code, &mut map);

		let file_len: u64 = counted_bytes.iter().sum();
		let file_len_bytes = file_len.to_le_bytes();
		writer.write_all(&file_len_bytes)?;
		//
		// Ważne !!! tutaj wartość 0 oznacza 256 !! 
		//
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

		reader.seek(SeekFrom::Start(0))?;
		for byte_read in reader.bytes() {
            let current_byte = byte_read?;
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
		
		if space < 8 {
            bufor = bufor << space;
            writer.write_all(&[bufor])?;
        }
		writer.flush()?;
		return Ok(())
	
	}

	fn decompress(&self, source: &mut dyn ReadSeek, destination: &mut dyn std::io::prelude::Write) -> std::io::Result<()> {
		let mut reader = BufReader::new(source);
		let mut writer = BufWriter::new(destination);
		
		let mut org_file_size_buf = [0;8];
		reader.read_exact(&mut org_file_size_buf)?;
		let org_file_size = u64::from_le_bytes(org_file_size_buf);

		let mut org_sign_count_buf = [0;1];
		let _ = reader.read_exact(&mut org_sign_count_buf);
		let org_sign_count = u8::from_le_bytes(org_sign_count_buf);

		let mut pq = BinaryHeap::new();
		

		for _ in 0..org_sign_count{
			let mut sign_buf = [0;1];
			let mut freq_buf = [0;8];

			let _ = reader.read_exact(&mut sign_buf);
			let _ = reader.read_exact(&mut freq_buf);

			let sign = u8::from_le_bytes(sign_buf);
			let freq = u64::from_le_bytes(freq_buf);
			let leaf = TreeNode::Leaf { byte: sign, freq:freq };
			pq.push((Reverse(freq),Box::new(leaf)));
			

		}

		while pq.len() > 1 {
			let (Reverse(freq1),left_node) = pq.pop().unwrap();
			let (Reverse(freq2),right_node) = pq.pop().unwrap();
			
			let combined_freq = freq1 + freq2;
			let parent = TreeNode::Root {
				freq: combined_freq,
				left: left_node, 
				right: right_node,
			};	
			pq.push((Reverse(combined_freq) ,Box::new(parent)));
		}
		let (Reverse(_total_freq), root_node) = pq.pop().unwrap();

		let mut current_node : &TreeNode = &root_node;
		let mut decoded_bytes: u64 = 0;

		'outer: for byte_read in reader.bytes(){

			let byte = byte_read?;

			for n in (0..8).rev() {
				if decoded_bytes == org_file_size {
                    break 'outer;
                }
			let current_bit : bool = ((byte >> n) & 1) >= 1;
			match current_node {
				TreeNode::Root { freq: _, left, right } => {
					if current_bit {
						current_node = &right;
					}
					else{
						current_node = &left;
					}
				}
				TreeNode::Leaf { .. } => {}
			}
			if let TreeNode::Leaf { byte: sign, .. } = current_node {
				writer.write_all(&[*sign])?;
				decoded_bytes += 1;
				current_node = &root_node; 
			}
			}
		}

		Ok(())
		
	}

	fn name(&self) -> &'static str {
		"Huffman Compression Algorithm"
	}

}

fn traverse(head : &TreeNode,current_code : BitCode, map : &mut HashMap<u8,BitCode>){
	match head {
		TreeNode::Root { left, right, .. } => {
			let left_code = BitCode {
				bits : current_code.bits << 1,
				len : current_code.len + 1
			};
			traverse(left,left_code,map);
			let right_code = BitCode{
				bits : (current_code.bits << 1) | 1,
				len : current_code.len + 1
			};
			traverse(right,right_code,map);

		}
		TreeNode::Leaf{byte,freq: _} => {
			map.insert(*byte,current_code);
		}
	}
}