use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Concatenate FILE(s) to standard output.
/// With no FILE, or when FILE is -, read standard input.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number all output lines
    #[arg(short = 'n', long, conflicts_with = "number_nonblank")]
    number: bool,

    /// Number nonempty output lines
    #[arg(short = 'b', long)]
    number_nonblank: bool,

    // The options -n and -b are mutually exclusive.
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Catch the Err variant and print the error message to STDERR.
    if let Err(e) = run(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }

    Ok(())
}

// Run the program with parsed arguments.
fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open_input_source(&filename) {
            Err(e) => {
                eprintln!("Failed to open {filename}: {e}")
            }
            Ok(file_content) => {
                // Initialize the line counter for each file.
                let mut line_count = 0;

                // Iterate through each line with index.
                for line in file_content.lines() {
                    // Shadow the line with the result of unpacking the Result.
                    let line = line?;

                    // Handle printing line numbers.
                    if args.number {
                        line_count += 1;
                        println!("{line_count:>6}\t{line}");

                        continue;
                    }

                    // Handle printing line numbers for non-blank lines.
                    if args.number_nonblank {
                        if line.is_empty() {
                            // Print a blank line.
                            println!();
                        } else {
                            line_count += 1;
                            println!("{line_count:>6}\t{line}");
                        }

                        continue;
                    }

                    // If there are no numbering options, just print the line.
                    println!("{line}");
                }
            }
        }
    }

    Ok(())
}

// Accepts a filename and returns either an error or a boxed value that implements the BufRead
// trait.
// - The return type includes the dyn keyword to say that the return type's trait is dynamically
// dispatched. This allows us to abstract the idea of the input source.
// - The return type is placed into a Box. which is a way to store a value on the heap. The
// compiler does not have enough information from dyn BufRead to know the size of the return type.
// If a variable does not have a fixed known size, then Rust cannot store it on the stack. The
// solution is to instead allocate memory on the heap by putting the return value into a Box, which
// is a pointer with a known size.
fn open_input_source(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
