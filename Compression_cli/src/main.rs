use clap::{Parser, Subcommand, ValueEnum};
use std::fs::File;
use std::fs::{metadata, Metadata};
use std::io::Write;
use std::path::PathBuf;

use compression_cli::algorithms::rle::RleCompression;
use compression_cli::algorithms::traits::CompressionAlgorithm;

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

fn get_file_size(file_path: &std::path::Path) -> Result<u64, std::io::Error> {
    let md = std::fs::metadata(file_path)?;
    Ok(md.len())
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

            let len_input: u64 = get_file_size(&input)?;
            let len_output: u64 = get_file_size(&output)?;

            println!("");
            println!("------------------------------------------------------");

            println!(
                "Udało się skompresować plik przy użyciu algorytmu {:?}",
                compression_algo
            );
            println!("------------------------------------------------------");
            println!("Początkowy rozmiar pliku: {} b", len_input);
            println!("Końcowy rozmiar pliku: {} b", len_output);
            println!(
                "Różnica rozmiarów: {}%",
                (len_input as f64 / len_output as f64) * 100.0
            );
            println!("------------------------------------------------------");
        }
        Commands::Audio {
            input,
            output,
            compression_algo,
            decompress,
        } => {}
    }

    Ok(())
}
