pub mod filter_reader;
pub mod png;

use crate::algorithms::traits::CompressionAlgorithm;
use crate::utils;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn process_image(
    input: &Path,
    output: &Path,
    decompress: bool,
    algo: &dyn CompressionAlgorithm,
) -> Result<(), Box<dyn std::error::Error>> {
    if decompress {
        decompress_image(input, output, algo)
    } else {
        compress_image(input, output, algo)
    }
}

fn compress_image(
    input: &Path,
    output: &Path,
    algo: &dyn CompressionAlgorithm,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input_file = File::open(input)?;
    let mut output_file = File::create(output)?;
    algo.compress(&mut input_file, &mut output_file)?;
    output_file.flush()?;
    utils::print_stats(input, output, Some(algo.name()));
    Ok(())
}

fn decompress_image(
    input: &Path,
    output: &Path,
    algo: &dyn CompressionAlgorithm,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input_file = File::open(input)?;
    let mut output_file = File::create(output)?;
    algo.decompress(&mut input_file, &mut output_file)?;
    output_file.flush()?;
    utils::print_stats(input, output, Some(algo.name()));
    Ok(())
}
