use crate::algorithms::traits::CompressionAlgorithm;
use std::collections::HashMap;
use std::io::{BufWriter, Read, Write};

pub struct LzwCompression;

impl CompressionAlgorithm for LzwCompression {
    fn compress(&self, source: &mut dyn Read, destination: &mut dyn Write) -> std::io::Result<()> {
        let mut writer = BufWriter::new(destination);

        let mut dict: HashMap<Vec<u8>, u32> = HashMap::new();

        for i in 0u32..256 {
            dict.insert(vec![i as u8], i);
        }
        let mut next_dict_val: u32 = 256;

        let mut input = Vec::new();
        source.read_to_end(&mut input)?;

        let mut current: Vec<u8> = Vec::new();

        for &byte in &input {
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

        let mut input = Vec::new();
        source.read_to_end(&mut input)?;

        let codes: Vec<u32> = input
            .chunks_exact(4)
            .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect();

        if codes.is_empty() {
            return Ok(());
        }

        let mut dict: Vec<Vec<u8>> = (0u32..256).map(|i| vec![i as u8]).collect();

        let mut previous = dict[codes[0] as usize].clone();
        writer.write_all(&previous)?;

        for &code in &codes[1..] {
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
