use clap::{Parser, Subcommand, ValueEnum};
use compression_cli::algorithms::rle::RleCompression;
use compression_cli::algorithms::traits::CompressionAlgorithm;
use compression_cli::audio;
use compression_cli::utils;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

//Tutaj dodajesz typy kompresowania
#[derive(ValueEnum, Clone, Debug)]
pub enum CompressionAlgo {
    Rle,
}

#[derive(Debug, Parser)]
#[command(name = "Compression CLI")]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    File {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,

        #[arg(short, long)]
        compression_algo: CompressionAlgo,

        #[arg(short, long)]
        decompress: bool,
    },
    Audio {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,

        #[arg(short, long)]
        compression_algo: CompressionAlgo,

        #[arg(short, long)]
        decompress: bool,
    },
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

            // todo!("validacja argumentów");

            match compression_algo {
                CompressionAlgo::Rle => {
                    let algo = RleCompression;
                    if decompress {
                        algo.decompress(&mut input_file, &mut output_file)?;
                    } else {
                        algo.compress(&mut input_file, &mut output_file)?;
                    }
                }
            }

            output_file.flush()?;

            utils::print_stats(&input, &output, Some(&format!("{:?}", compression_algo)));
        }
        Commands::Audio {
            input,
            output,
            compression_algo,
            decompress,
        } => match compression_algo {
            CompressionAlgo::Rle => {
                let algo = RleCompression;
                audio::process_audio(&input, &output, decompress, &algo);
            }
        },
    }

    Ok(())
}
