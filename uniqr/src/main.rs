use anyhow::{anyhow, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

/// Report or omit repeated lines
#[derive(Debug, Parser, Clone)]
#[command(author, version, about)]
struct Args {
    /// Input file
    #[arg(value_name = "INPUT", default_value = "-")]
    in_file: String,

    /// Output file
    #[arg(value_name = "OUTPUT")]
    out_file: Option<String>,

    /// Prefix lines by the number of occurrences
    #[arg(short, long)]
    count: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Err(e) = do_run(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }

    Ok(())
}

fn do_run(args: Args) -> Result<()> {
    // Create an informative error message on failure.
    let mut filehandle =
        open_input_source(&args.in_file).map_err(|e| anyhow!("{}: {}", args.in_file, e))?;

    // Create a new empty mutable string buffer to hold each line.
    let mut line_buffer = String::new();

    // Read lines of text from an input file or STDIN, preserving the line endings.
    loop {
        let bytes_read = filehandle.read_line(&mut line_buffer)?;

        if bytes_read == 0 {
            break;
        }

        print!("{line_buffer}");

        line_buffer.clear();
    }

    Ok(())
}

fn open_input_source(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
