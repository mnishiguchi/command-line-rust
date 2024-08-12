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

    let print_info_row = |n: u64, s: &str| {
        // Print the output only when count is greater than 0.
        if n > 0 {
            if args.count {
                print!("{:>4} {}", n, s);
            } else {
                print!("{}", s);
            }
        }
    };

    let mut current_line = String::new();
    let mut previous_line = String::new();
    let mut duplicate_count: u64 = 0;

    // Read lines of text from an input file or STDIN, preserving the line endings.
    loop {
        let bytes_read = filehandle.read_line(&mut current_line)?;

        if bytes_read == 0 {
            break;
        }

        let is_different_from_previous = current_line.trim_end() != previous_line.trim_end();

        if is_different_from_previous {
            print_info_row(duplicate_count, &previous_line);
            previous_line = current_line.clone();
            duplicate_count = 0;
        }

        duplicate_count += 1;
        current_line.clear();
    }

    print_info_row(duplicate_count, &previous_line);

    Ok(())
}

fn open_input_source(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
