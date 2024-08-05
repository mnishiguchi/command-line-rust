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
            Err(e) => {
                eprintln!("{filename}: {e}");
            }
            // Accept the filehandle as a mutable value.
            Ok(mut filehandle) => {
                // Check if args.bytes is some number of bytes to read.
                if let Some(byte_count) = args.bytes {
                    // This branch is to support the BYTES option.

                    // Create a mutable buffer of a fixed length filled with zeros to hold the bytes
                    // read from the file.
                    let mut buffer = vec![0; byte_count as usize];

                    // The value will contain the number of bytes that were read, which can be
                    // fewer than the number requested.
                    let bytes_read = filehandle.read(&mut buffer)?;

                    // Convert the selected bytes into a string, which can be invalid UTF-8.
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    // Create a new empty mutable string buffer to hold each line.
                    let mut line = String::new();

                    // Iterate through a std::ops::Range to count up from zero to the requested number
                    // of lines.
                    for _ in 0..args.lines {
                        // Read the next line into the string buffer.
                        let bytes_read = filehandle.read_line(&mut line)?;

                        // Break out of the loop when reaching the end of the file.
                        if bytes_read == 0 {
                            break;
                        }

                        // Print the line including the original line ending.
                        print!("{line}");

                        // Empty the line buffer.
                        line.clear();
                    }
                }
            }
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
