mod chart;
mod io;
mod pipeline;
pub mod signal;

use crate::algorithms::traits::CompressionAlgorithm;
use std::path::PathBuf;

pub fn process_audio(
    input: &PathBuf,
    output: &PathBuf,
    decompress_flag: bool,
    algo: &dyn CompressionAlgorithm,
) {
    if decompress_flag {
        pipeline::decompress(input, output, algo);
    } else {
        pipeline::compress(input, output, algo);
    }
}
