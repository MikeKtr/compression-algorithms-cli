use crate::algorithms::traits::{CompressionAlgorithm, ReadSeek};
use std::io::{BufReader, BufWriter, Read, Write};

pub struct RleCompression;

impl CompressionAlgorithm for RleCompression {
    fn compress(
        &self,
        source: &mut dyn ReadSeek,
        destination: &mut dyn Write,
    ) -> std::io::Result<()> {
        let reader = BufReader::new(source);
        let mut writer = BufWriter::new(destination);

        let mut current_byte: Option<u8> = None;
        let mut counter: u8 = 0;

        for byte_read in reader.bytes() {
            let byte = byte_read?;
            if current_byte.is_none() || current_byte == Some(byte) {
                counter += 1;
                if counter == 255 {
                    writer.write_all(&[counter, byte])?;
                    counter = 0;
                    current_byte = None;
                    continue;
                }
            } else {
                if let Some(raw_byte) = current_byte {
                    writer.write_all(&[counter, raw_byte])?;
                }
                counter = 1;
            }
            current_byte = Some(byte);
        }
        if let Some(raw_byte) = current_byte {
            writer.write_all(&[counter, raw_byte])?;
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

        let mut buffer = [0u8; 2];

        while reader.read_exact(&mut buffer).is_ok() {
            let number_byte = buffer[0];
            let letter_byte = buffer[1];

            for _i in 0..number_byte {
                let _ = writer.write_all(&[letter_byte]);
            }
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Rle Compression"
    }

    fn description(&self) -> &'static str {
        "Compresses data by replacing consecutive identical bytes with a [count, value] pair.\nExample: 'AAAAABBB' is compressed into [5, 'A', 3, 'B']"
    }
}
