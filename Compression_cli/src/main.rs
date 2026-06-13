use clap::{Parser, Subcommand, ValueEnum};
use compression_cli::algorithms::huffman_tree::HuffmanCompression;
use compression_cli::algorithms::lzw::LzwCompression;
use compression_cli::algorithms::rle::RleCompression;
use compression_cli::algorithms::traits::CompressionAlgorithm;
use compression_cli::audio;
use compression_cli::image;
use compression_cli::image::png::PngCompression;
use compression_cli::utils;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(ValueEnum, Clone, Debug)]
pub enum CompressionAlgo {
    Rle,
    HuffmanCompression,
    Lzw,
    Png,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    File {
        input: PathBuf,
        output: PathBuf,
        compression_algo: CompressionAlgo,
        #[arg(short, long, default_value_t = false)]
        decompress: bool,
    },
    Audio {
        input: PathBuf,
        output: PathBuf,
        compression_algo: CompressionAlgo,
        #[arg(short, long, default_value_t = false)]
        decompress: bool,
    },
    Image {
        input: PathBuf,
        output: PathBuf,
        #[arg(short, long, default_value_t = false)]
        decompress: bool,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Compression Cli", long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    match args.command {
        Commands::File {
            input,
            output,
            compression_algo,
            decompress,
        } => {
            let mut input_file = File::open(&input)?;
            let mut output_file = File::create(&output)?;

            let algo: &dyn CompressionAlgorithm = match compression_algo {
                CompressionAlgo::Rle => &RleCompression,
                CompressionAlgo::HuffmanCompression => &HuffmanCompression,
                CompressionAlgo::Png => &PngCompression,
                CompressionAlgo::Lzw => &LzwCompression,
            };

            if decompress {
                algo.decompress(&mut input_file, &mut output_file)?;
            } else {
                algo.compress(&mut input_file, &mut output_file)?;
            }

            output_file.flush()?;
            utils::print_stats(&input, &output, Some(&format!("{:?}", compression_algo)));
        }

        Commands::Audio {
            input,
            output,
            compression_algo,
            decompress,
        } => {
            let algo: &dyn CompressionAlgorithm = match compression_algo {
                CompressionAlgo::Rle => &RleCompression,
                CompressionAlgo::Lzw => &LzwCompression,
                CompressionAlgo::HuffmanCompression => &HuffmanCompression,
                CompressionAlgo::Png => &PngCompression,
            };
            audio::process_audio(&input, &output, decompress, algo);
        }

        Commands::Image {
            input,
            output,
            decompress,
        } => {
            let algo = &PngCompression;
            image::process_image(&input, &output, decompress, algo)?;
        }
    }

    Ok(())
}
