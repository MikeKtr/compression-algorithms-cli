use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};

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

pub fn traverse(head : &TreeNode,current_code : BitCode, map : &mut HashMap<u8,BitCode>){
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

pub fn create_tree(counted_bytes : &mut [u64;256],map: &mut HashMap<u8,BitCode>){

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

	// let mut map = HashMap::new();
	let initial_code = BitCode { bits: 0, len: 0 };

	traverse(&root_node, initial_code, map);

}