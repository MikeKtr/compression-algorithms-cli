use crate::algorithms::traits::CompressionAlgorithm;
use std::collections::HashMap;
use std::io::{BufWriter, Read, Write};

pub struct LzwCompression;

impl CompressionAlgorithm for LzwCompression {
    fn compress(&self, source: &mut dyn Read, destination: &mut dyn Write) -> std::io::Result<()> {
        let source_reader = std::io::BufReader::new(source);
        let mut writer = BufWriter::new(destination);

        let mut dict: HashMap<Vec<u8>, u32> = HashMap::new();

        for i in 0u32..256 {
            dict.insert(vec![i as u8], i);
        }
        let mut next_dict_val: u32 = 256;

        let mut current: Vec<u8> = Vec::new();

        for byte_result in source_reader.bytes() {
            let byte = byte_result?;
            let mut extended = current.clone();
            extended.push(byte);

            if dict.contains_key(&extended) {
                current = extended;
            } else {
                let code = *dict.get(&current).unwrap();
                writer.write_all(&code.to_le_bytes())?;

                dict.insert(extended, next_dict_val);
                next_dict_val += 1;

                current = vec![byte];
            }
        }

        if !current.is_empty() {
            let code = *dict.get(&current).unwrap();
            writer.write_all(&code.to_le_bytes())?;
        }

        writer.flush()?;
        Ok(())
    }

    fn decompress(
        &self,
        source: &mut dyn Read,
        destination: &mut dyn Write,
    ) -> std::io::Result<()> {
        let mut writer = BufWriter::new(destination);
        let mut dict: Vec<Vec<u8>> = (0u32..256).map(|i| vec![i as u8]).collect();

        let mut buf = [0u8; 4];

        if source.read_exact(&mut buf).is_err() {
            return Ok(());
        }
        let first_code = u32::from_le_bytes(buf);

        let mut previous = dict[first_code as usize].clone();
        writer.write_all(&previous)?;

        while let Ok(_) = source.read_exact(&mut buf) {
            let code = u32::from_le_bytes(buf);

            let entry = if (code as usize) < dict.len() {
                dict[code as usize].clone()
            } else {
                let mut entry = previous.clone();
                entry.push(previous[0]);
                entry
            };

            writer.write_all(&entry)?;

            let mut new_entry = previous.clone();
            new_entry.push(entry[0]);
            dict.push(new_entry);

            previous = entry;
        }

        writer.flush()?;
        Ok(())
    }

    fn name(&self) -> &'static str {
        "LZW Compression"
    }
}
