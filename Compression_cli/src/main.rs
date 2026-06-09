use clap::{Parser, Subcommand, ValueEnum};
use compression_cli::algorithms::lzw::LzwCompression;
use compression_cli::algorithms::rle::RleCompression;
use compression_cli::algorithms::traits::CompressionAlgorithm;
use compression_cli::audio;
use compression_cli::utils;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

	use std::path::PathBuf;
	use clap::{Parser, ValueEnum};
	use std::fs::File;

	use std::io::Write;

	use compression_cli::algorithms::traits::CompressionAlgorithm;
	use compression_cli::algorithms::rle::RleCompression;
	use compression_cli::algorithms::huffman_tree::HuffmanCompression;
	use compression_cli::algorithms::png::PngCompression;



	//Tutaj dodajesz typy kompresowania
	#[derive(ValueEnum,Clone,Debug)]
	pub enum CompressionAlgo{
		Rle,
		HuffmanCompression,
		Png
	}

	#[derive(Debug,Parser)]
	pub struct CliArgs{
		#[arg(short,long)]
		pub input: PathBuf,

		#[arg(short,long)]
		pub output: PathBuf,

		#[arg(short,long)]
		pub compression_algo : CompressionAlgo,

		#[arg(short,long)]
		pub decompress : bool,
	}



	fn get_file_size(file_path : &std::path::Path) -> Result<u64, std::io::Error>{
		let md = std::fs::metadata(file_path)?;
		Ok(md.len())
	}

	fn main() -> Result<(), Box<dyn std::error::Error>>{

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

		match args.compression_algo {
			CompressionAlgo::Rle => {
				let algo = RleCompression;
				if args.decompress{
					algo.decompress(&mut input_file, &mut output_file)?;
				}
				else{
					algo.compress(&mut input_file, &mut output_file)?;
				}
			}
			CompressionAlgo::HuffmanCompression => {
				let algo = HuffmanCompression;
				if args.decompress{
					algo.decompress(&mut input_file, &mut output_file)?;
				}
				else{
					algo.compress(&mut input_file, &mut output_file)?;
				}
			}
			CompressionAlgo::Png =>{
				let algo = PngCompression;
				if args.decompress{
					algo.decompress(&mut input_file, &mut output_file)?;
				}
				else{
					algo.compress(&mut input_file, &mut output_file)?;
				}
			}
		}

		output_file.flush()?;

		let len_input: u64 = get_file_size(&args.input)?;
		let len_output: u64 = get_file_size(&args.output)?;

		println!("");
		println!("------------------------------------------------------");

		println!("Udało się skompresować plik przy użyciu algorytmu {:?}",args.compression_algo);
		println!("------------------------------------------------------");
		println!("Początkowy rozmiar pliku: {} b",len_input);
		println!("Końcowy rozmiar pliku: {} b",len_output);
		println!("Różnica rozmiarów: {}%",(len_output as f64 / len_input as f64) * 100.0);
		println!("------------------------------------------------------");

            match compression_algo {
                CompressionAlgo::Rle => {
                    let algo = RleCompression;
                    if decompress {
                        algo.decompress(&mut input_file, &mut output_file)?;
                    } else {
                        algo.compress(&mut input_file, &mut output_file)?;
                    }
                }
                CompressionAlgo::Lzw => {
                    let algo = LzwCompression;
                    if decompress {
                        algo.decompress(&mut input_file, &mut output_file)?;
                    } else {
                        algo.compress(&mut input_file, &mut output_file)?;
                    }
                }
            }

            output_file.flush()?;
            if !decompress {
                utils::print_stats(&input, &output, Some(&format!("{:?}", compression_algo)));
            }
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
            CompressionAlgo::Lzw => {
                let algo = LzwCompression;
                audio::process_audio(&input, &output, decompress, &algo);
            }
        },
    }

    Ok(())
}
