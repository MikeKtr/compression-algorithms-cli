use std::fs::File;
use std::io::Cursor;
use std::path::Path;

use crate::algorithms::rle::RleCompression;
use crate::algorithms::traits::CompressionAlgorithm;

use super::chart::show_waveform;
use super::io::{load_samples, save_samples};
use super::signal::{delta_decode, delta_encode, quantize_samples};

pub fn compress(input: &Path, output: &Path) {
    let mut samples = match load_samples(input) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Błąd: {}", e);
            return;
        }
    };

    show_waveform(&samples);

    quantize_samples(&mut samples);
    let deltas = delta_encode(&samples);

    let byte_data: Vec<u8> = deltas.iter().flat_map(|&s| s.to_le_bytes()).collect();

    let mut source = Cursor::new(byte_data);
    let mut output_file = File::create(output).expect("Błąd tworzenia pliku wyjściowego");

    RleCompression
        .compress(&mut source, &mut output_file)
        .expect("Błąd kompresji RLE");

    print_stats(input, output);
}

pub fn decompress(input: &Path, output: &Path) {
    let mut input_file = File::open(input).expect("Błąd otwierania pliku do dekompresji");

    let mut decompressed_bytes = Vec::new();
    let mut out_cursor = Cursor::new(&mut decompressed_bytes);

    RleCompression
        .decompress(&mut input_file, &mut out_cursor)
        .expect("Błąd dekompresji RLE");

    let deltas: Vec<i16> = decompressed_bytes
        .chunks_exact(2)
        .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    let samples = delta_decode(&deltas);

    show_waveform(&samples);

    if let Err(e) = save_samples(output, &samples) {
        eprintln!("Błąd zapisu: {}", e);
    }
}

fn print_stats(input: &Path, output: &Path) {
    let original_size = std::fs::metadata(input).unwrap().len();
    let compressed_size = std::fs::metadata(output).unwrap().len();
    let ratio = (1.0 - compressed_size as f64 / original_size as f64) * 100.0;

    println!("\n--- SUKCES ---");
    println!("Rozmiar przed: {} bajtów", original_size);
    println!("Rozmiar po:    {} bajtów", compressed_size);
    println!("Zredukowano:   {:.2}%", ratio);
}
