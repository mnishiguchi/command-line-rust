use clap::{ArgAction, Parser};

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

    Ok(())
}
