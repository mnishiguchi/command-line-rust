use clap::{ArgAction, Parser};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

/// compare two sorted files line by line
#[derive(Debug, clap::Parser, Clone)]
#[command(author, version, about)]
struct CliArguments {
    // Positional arguments
    //
    // - The order in which positional arguments are defined is important.
    //
    /// Input file 1
    #[arg()]
    file1: String,

    /// Input file 2
    #[arg()]
    file2: String,

    //  Optional arguments
    //
    //  - The order in which optional arguments are defined does not matter.
    //
    /// Suppress printing of column 1 (lines unique to FILE1)
    #[arg(short='1', action=ArgAction::SetFalse)]
    show_col1: bool,

    /// Suppress printing of column 2 (lines unique to FILE2)
    #[arg(short='2', action=ArgAction::SetFalse)]
    show_col2: bool,

    /// Suppress printing of column 3 (lines that appear in both files)
    #[arg(short='3', action=ArgAction::SetFalse)]
    show_col3: bool,

    /// Ignore case distinctions when comparing lines
    #[arg(short, long)]
    ignore_case: bool,

    /// Separate columns with DELIMITER
    #[arg(short, long = "output-delimiter", default_value = "\t")]
    delimiter: String,
}

fn main() {
    if let Err(e) = do_run(CliArguments::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn do_run(args: CliArguments) -> anyhow::Result<()> {
    println!("{:#?}", args);

    let file1 = &args.file1;
    let file2 = &args.file2;

    // Prohibit that both the filenames being "-"
    if file1 == "-" && file2 == "-" {
        anyhow::bail!(r#"Both input files cannot be STDIN ("-")"#);
    }

    // Attempt to open the two input files
    let _filehandle1 = open_input_file(file1)?;
    let _filehandle2 = open_input_file(file2)?;
    println!(r#"Opened "{file1}" and "{file2}""#);

    Ok(())
}

// Opening user-provided input source
fn open_input_file(filename: &str) -> anyhow::Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => {
            // Incorporate the filename into the error message
            Ok(Box::new(BufReader::new(
                File::open(filename).map_err(|e| anyhow::anyhow!("{filename}: {e}"))?,
            )))
        }
    }
}
