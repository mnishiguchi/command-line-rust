use anyhow::Result;
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

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
      long,
      conflicts_with = "lines",
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
    let file_count = args.files.len();

    for (file_index, filename) in args.files.iter().enumerate() {
        match open_input_source(&filename) {
            Err(e) => {
                eprintln!("{filename}: {e}");
            }
            // Accept the filehandle as a mutable value.
            Ok(mut filehandle) => {
                // Only print headers when there are multiple files.
                if file_count > 1 {
                    let linebreak = if file_index > 0 { "\n" } else { "" };
                    println!("{linebreak}==> {filename} <==")
                }

                // Check if args.bytes is some number of bytes to read.
                if let Some(requested_byte_count) = args.bytes {
                    // This branch is to support the BYTES option.

                    // Read the desired number of bytes from a file. Be sure to add to our imports the trait
                    // std::io::Read. We must indicate that we want a Vec (size known), not a slice
                    // (size unknown).
                    let bytes_read = filehandle
                        .bytes()
                        .take(requested_byte_count as usize)
                        .collect::<Result<Vec<_>, _>>()?;

                    // Convert the selected bytes into a string, which can be invalid UTF-8.
                    // The size for bytes must be known at complile-time.
                    print!("{}", String::from_utf8_lossy(&bytes_read));
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
