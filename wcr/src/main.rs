use anyhow::Result;
use clap::Parser;

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

fn run(args: Args) -> Result<()> {
    println!("{args:#?}");

    Ok(())
}
