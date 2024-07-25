use anyhow::{Context, Result};
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

// Box<dyn std::error::Error> can contain any type that implements the standard Error trait. So we
// can use `?` on all of the usual functions that return Results.
fn main() -> Result<()> {
    // Example
    //   $ cargo run --quiet -- main src/main.rs

    // Cli::parse() is meant to be used in our main(); don't use it in other places.
    let args: Cli = Cli::parse();

    let f: File =
        File::open(&args.path).with_context(|| format!("could not read file `{:?}`", args.path))?;

    let _: Result<()> = find_matches(f, &args.pattern, &mut std::io::stdout());

    Ok(())
}

fn find_matches(f: File, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    // BufReader.lines() reads a file more efficiently than std::fs::read_to_string().
    let reader: BufReader<File> = BufReader::new(f);

    for line in reader.lines() {
        // With a question mark, Rust will internally expand the Result.
        let s = line?;

        if s.contains(&pattern) {
            // writeln!() returns an io::Result because writing can fail.
            writeln!(writer, "{}", s)?;
        }
    }

    // This means "Result is OK and has no content."
    // The last expression of any block in Rust is its return value.
    Ok(())
}
