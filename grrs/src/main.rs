use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::stdout;
use std::path::PathBuf;

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    // PathBuf is like a String but for file system paths that work cross-platform.
    path: PathBuf,
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

    let _: Result<()> = grrs::find_matches(f, &args.pattern, &mut stdout());

    Ok(())
}
