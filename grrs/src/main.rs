use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    // PathBuf is like a String but for file system paths that work cross-platform.
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example
    //   $ cargo run --quiet -- main src/main.rs

    // Cli::parse() is meant to be used in our main(); don't use it in other places.
    let args: Cli = Cli::parse();

    // BufReader.lines() reads a file more efficiently than std::fs::read_to_string().
    let f: File = File::open(&args.path).expect("could not read file");
    let reader: BufReader<File> = BufReader::new(f);

    for line in reader.lines() {
        let s: String = match line {
            Ok(content) => content,
            Err(error) => {
                return Err(error.into());
            }
        };

        if s.contains(&args.pattern) {
            println!("{}", s);
        }
    }

    // This means "Result is OK and has no content."
    // The last expression of any block in Rust is its return value.
    Ok(())
}
