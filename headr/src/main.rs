use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Print the first 10 lines of each FILE to standard output.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number of lines
    #[arg(
      short = 'n',
      long,
      default_value = "10",
      value_parser = clap::value_parser!(u64).range(1..),
    )]
    lines: u64,

    /// Number of bytes
    #[arg(
      short = 'c',
      long, conflicts_with = "lines",
      value_parser = clap::value_parser!(u64).range(1..),
    )]
    bytes: Option<u64>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }

    Ok(())
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open_input_source(&filename) {
            Err(e) => eprintln!("{filename}: {e}"),
            Ok(_) => println!("Opened {filename}"),
        }
    }

    Ok(())
}

fn open_input_source(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
