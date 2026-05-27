
	use std::path::PathBuf;
	use clap::{Parser, ValueEnum};
	use std::fs::File;
	use std::fs::{Metadata,metadata};
	use std::io::Write;
	
	use compression_cli::algorithms::traits::CompressionAlgorithm;
	use compression_cli::algorithms::rle::RleCompression;


	//Tutaj dodajesz typy kompresowania
	#[derive(ValueEnum,Clone,Debug)]
	pub enum CompressionAlgo{
		Rle
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

		let mut input_file = File::open(&args.input)?;
		let mut output_file = File::create(&args.output)?;



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
		println!("Różnica rozmiarów: {}%",(len_input as f64 / len_output as f64) * 100.0);
		println!("------------------------------------------------------");


		Ok(())
	}
