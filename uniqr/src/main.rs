use anyhow::Result;
use clap::Parser;

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

    if let Err(e) = run(args.clone()) {
        eprintln!("{e}");
        std::process::exit(1);
    }

    println!("{:?}", args);
    Ok(())
}

fn run(_args: Args) -> Result<()> {
    Ok(())
}
