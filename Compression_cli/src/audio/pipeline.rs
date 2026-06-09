use std::fs::File;
use std::io::Cursor;
use std::path::Path;

use crate::algorithms::traits::CompressionAlgorithm;
use crate::utils;

use super::chart::show_waveform;
use super::io::{load_samples, save_samples};
use super::signal::{delta_decode, delta_encode, quantize_samples};

pub fn compress(input: &Path, output: &Path, algo: &dyn CompressionAlgorithm) {
    let mut samples = match load_samples(input) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("blad: {}", e);
            return;
        }
    };

    show_waveform(&samples);

    quantize_samples(&mut samples);

    let deltas = delta_encode(&samples);

    let byte_data: Vec<u8> = deltas.iter().flat_map(|&s| s.to_le_bytes()).collect();

    let mut source = Cursor::new(byte_data);
    let mut output_file = File::create(output).expect("blad tworzenia pliku wyjsciowego");

    algo.compress(&mut source, &mut output_file)
        .expect("Błąd kompresji algorytmu audio");

    utils::print_stats(input, output, Some(algo.name()));
}

pub fn decompress(input: &Path, output: &Path, algo: &dyn CompressionAlgorithm) {
    let mut input_file = File::open(input).expect("blad otwierania pliku do dekompresji");

    let mut decompressed_bytes = Vec::new();
    let mut out_cursor = Cursor::new(&mut decompressed_bytes);

    algo.decompress(&mut input_file, &mut out_cursor)
        .expect("Błąd dekompresji algorytmu audio");

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
