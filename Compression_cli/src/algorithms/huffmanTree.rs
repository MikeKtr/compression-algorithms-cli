use std::{cmp::Reverse, collections::{BinaryHeap, binary_heap}, io::{BufReader, BufWriter, Read}};
use crate::algorithms::traits::CompressionAlgorithm;

#[derive(Debug)]
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

pub struct HuffmanCompression;

impl CompressionAlgorithm for HuffmanCompression{
	fn compress(&self,source: &mut dyn std::io::prelude::Read, destination: &mut dyn std::io::prelude::Write) ->  std::io::Result<()> {
		let mut reader = BufReader::new(source);
		let mut writer = BufWriter::new(destination);

		let mut current_byte : Option<u8> = None;
		let mut counted_bytes  = [0u64; 256];
		for byte_read in reader.bytes(){
			let byte = byte_read?;
			counted_bytes[byte as usize] += 1;
		}
		let Some(&max_val) = counted_bytes.iter().max();

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
			pq.push((Reverse(combined_freq),Box::new(parent)));
		}
		let (Reverse(_total_freq), root_node) = pq.pop().unwrap();
	Ok(())
			
	}

	fn decompress(&self, source: &mut dyn std::io::prelude::Read, destination: &mut dyn std::io::prelude::Write) -> std::io::Result<()> {
		todo!()
	}

	fn name(&self) -> &'static str {
		"Huffman Compression Algorithm"
	}
}