use std::io::{Read,Write,BufReader,BufWriter};

pub fn compress(source: &mut dyn Read,destination : &mut dyn Write) -> std::io::Result<()>{
	let mut reader = BufReader::new(source);
	let mut writer = BufWriter::new(destination);

	let mut current_byte : Option<u8> = None;
	let mut counter : u8 = 0;

	for byte_read in reader.bytes(){
		let byte = byte_read?;
		if current_byte == None || current_byte == Some(byte){
			counter+=1
		}
		else{

			if let Some(raw_byte) = current_byte {
                writer.write_all(&[counter, raw_byte])?; 
            }
			counter = 0;
			
		}
		current_byte = Some(byte);
	}
	if let Some(raw_byte) = current_byte {
        writer.write_all(&[counter, raw_byte])?;
    }
	writer.flush()?;
	Ok(())
}
