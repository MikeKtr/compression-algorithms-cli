use crate::algorithms::traits::{CompressionAlgorithm, ReadSeek};
use std::collections::HashMap;
use std::io::{BufWriter, Read, Write};

pub struct LzwCompression;

impl CompressionAlgorithm for LzwCompression {
    fn compress(
        &self,
        source: &mut dyn ReadSeek,
        destination: &mut dyn Write,
    ) -> std::io::Result<()> {
        let source_reader = std::io::BufReader::new(source);
        let mut writer = BufWriter::new(destination);

        let mut dict: HashMap<(u16, u8), u16> = HashMap::new();
        let mut next_code: u16 = 256;
        let max_code: u16 = 65535;

        let mut current_prefix: Option<u16> = None;

        for byte_result in source_reader.bytes() {
            let k = byte_result?;

            if let Some(w) = current_prefix {
                if let Some(&code) = dict.get(&(w, k)) {
                    current_prefix = Some(code);
                } else {
                    writer.write_all(&w.to_le_bytes())?;
                    if next_code < max_code {
                        dict.insert((w, k), next_code);
                        next_code += 1;
                    }
                    current_prefix = Some(k as u16);
                }
            } else {
                current_prefix = Some(k as u16);
            }
        }

        if let Some(w) = current_prefix {
            writer.write_all(&w.to_le_bytes())?;
        }

        writer.flush()?;
        Ok(())
    }

    fn decompress(
        &self,
        source: &mut dyn ReadSeek,
        destination: &mut dyn Write,
    ) -> std::io::Result<()> {
        let mut writer = BufWriter::new(destination);

        let mut dict: Vec<Vec<u8>> = (0u32..256).map(|i| vec![i as u8]).collect();
        let max_code: usize = 65535;

        let mut buf = [0u8; 2];

        if source.read_exact(&mut buf).is_err() {
            return Ok(());
        }

        let first_code = u16::from_le_bytes(buf) as usize;
        if first_code >= dict.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Wrong lzw file",
            ));
        }

        let mut previous = dict[first_code].clone();
        writer.write_all(&previous)?;

        while source.read_exact(&mut buf).is_ok() {
            let code = u16::from_le_bytes(buf) as usize;

            let entry = if code < dict.len() {
                dict[code].clone()
            } else if code == dict.len() {
                let mut entry = previous.clone();
                entry.push(previous[0]);
                entry
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Corrupted file",
                ));
            };

            writer.write_all(&entry)?;

            if dict.len() < max_code {
                let mut new_entry = previous.clone();
                new_entry.push(entry[0]);
                dict.push(new_entry);
            }

            previous = entry;
        }

        writer.flush()?;
        Ok(())
    }

    fn name(&self) -> &'static str {
        "LZW Compression"
    }

    fn description(&self) -> &'static str {
        "Lempel-Ziv-Welch (LZW):\n A dictionary-based algorithm that dynamically maps repeating sequences to 16-bit codes.\n Example: In 'TOBEORNOTTOBE', the second 'TOBE' is already in the dictionary and gets replaced by a single short code"
    }
}
