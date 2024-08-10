use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Print newline, word, and byte counts for each FILE, and a total line if more than one FILE is
/// specified.  A word is a non-zero-length sequence of printable characters delimited by white
/// space.
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Input files(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Show line count
    #[arg(short, long)]
    lines: bool,

    /// Show word count
    #[arg(short, long)]
    words: bool,

    /// Show byte count
    #[arg(short = 'c', long)]
    bytes: bool,

    /// Show character count
    #[arg(short = 'm', long, conflicts_with = "bytes")]
    chars: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }

    Ok(())
}

fn run(mut args: Args) -> Result<()> {
    // Check if all the flags are false
    let are_all_flags_false = [args.words, args.bytes, args.chars, args.lines]
        // Create an iterator.
        .iter()
        // Compare to &false because the values are references.
        .all(|v| v == &false);

    // Assign default settings if all flags are false.
    if are_all_flags_false {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    println!("{args:#?}");

    for filename in &args.files {
        match open_input_source(filename) {
            Err(e) => {
                eprintln!("{filename}: {e}")
            }
            Ok(_) => {
                println!("Opened {filename}")
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
