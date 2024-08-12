use anyhow::{anyhow, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
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
    let mut in_filehandle =
        open_input_file(&args.in_file).map_err(|e| anyhow!("{}: {}", args.in_file, e))?;

    let mut out_filehandle: Box<dyn Write> =
        open_output_file(&args.out_file).map_err(|e| anyhow!("{:?}: {}", args.out_file, e))?;

    // This closure must be declared as mutable because the out_filehandle is borrowed as a mutable
    // value.
    let mut print_info_row = |n: u64, s: &str| -> Result<()> {
        // Print the output only when count is greater than 0.
        if n > 0 {
            if args.count {
                write!(out_filehandle, "{:>4} {}", n, s)?;
            } else {
                write!(out_filehandle, "{}", s)?;
            }
        }

        Ok(())
    };

    // These buffers allow us to only allocate memory for the current and previout lines so our
    // program can scale to any file size.
    let mut current_line = String::new();
    let mut previous_line = String::new();
    let mut duplicate_count: u64 = 0;

    // Read lines of text from an input file or STDIN, preserving the line endings.
    loop {
        let bytes_read = in_filehandle.read_line(&mut current_line)?;

        if bytes_read == 0 {
            break;
        }

        let is_different_from_previous = current_line.trim_end() != previous_line.trim_end();

        if is_different_from_previous {
            print_info_row(duplicate_count, &previous_line)?;
            previous_line = current_line.clone();
            duplicate_count = 0;
        }

        duplicate_count += 1;
        current_line.clear();
    }

    print_info_row(duplicate_count, &previous_line)?;

    Ok(())
}

fn open_input_file(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        path => Ok(Box::new(BufReader::new(File::open(path)?))),
    }
}

fn open_output_file(filename: &Option<String>) -> Result<Box<dyn Write>> {
    match filename {
        None => Ok(Box::new(io::stdout())),
        Some(path) => Ok(Box::new(File::create(path)?)),
    }
}
