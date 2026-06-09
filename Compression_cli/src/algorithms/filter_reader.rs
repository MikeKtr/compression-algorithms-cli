use std::io::{Read,BufReader};

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


        for i in 0..self.img_width {
            let left = if i >= 3 { current_line[i - 3] } else { 0 };
            let up = self.prev_line[i]; 
        
            let prediction = ((left as u16 + up as u16) / 2) as u8;
        
            filtered_line[i + 1] = current_line[i].wrapping_sub(prediction);
        }

			self.prev_line.copy_from_slice(&current_line);

		


        self.current_row += 1;

        Some(Ok(filtered_line))
    }
}