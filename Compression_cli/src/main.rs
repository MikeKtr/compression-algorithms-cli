
use std::path::PathBuf;
use clap::{Parser, ValueEnum};
use std::fs::File;

use compression_cli::algorithms;

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


fn main() -> Result<(), Box<dyn std::error::Error>>{

	let args = CliArgs::parse();

	let mut input_file = File::open(&args.input)?;
    let mut output_file = File::create(&args.output)?;

	// todo!("validacja argumentów");

	match args.compression_algo {
		CompressionAlgo::Rle => {
			if args.decompress{
				todo!("dokompresowanie");
			}
			else{
				algorithms::rle::compress(&mut input_file, &mut output_file)?;
			}
		}
	}
	println!("Wynik zapisano w {:?}", args.output);

	Ok(())
}
