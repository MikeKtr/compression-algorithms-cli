mod chart;
mod io;
mod pipeline;
pub mod signal;

use std::path::PathBuf;

pub fn process_audio(input: &PathBuf, output: &PathBuf, decompress_flag: bool) {
    if decompress_flag {
        pipeline::decompress(input, output);
    } else {
        pipeline::compress(input, output);
    }
}
