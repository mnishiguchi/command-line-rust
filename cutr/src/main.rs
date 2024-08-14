use anyhow::Result;
use clap::Parser;

/// Remove sections from each line of files.
#[derive(Debug, clap::Parser, Clone)]
#[command(author, version, about)]
struct CliArguments {
    /// Input file
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Field delimiter
    #[arg(short, long, default_value = "\t")]
    delimiter: String,

    // NOTE: The flatten command will merge the SelectionArguments in the CliArguments struct.
    #[command(flatten)]
    selection_arguments: SelectionArguments,
}

#[derive(Debug, clap::Args, Clone)]
#[group(required = true, multiple = false)]
struct SelectionArguments {
    /// Selected fields
    #[arg(short, long)]
    fields: Option<String>,

    /// Selected bytes
    #[arg(short, long)]
    bytes: Option<String>,

    /// Selected characters
    #[arg(short, long)]
    chars: Option<String>,
}

fn main() -> Result<()> {
    let args = CliArguments::parse();

    if let Err(e) = do_run(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }

    Ok(())
}

fn do_run(args: CliArguments) -> Result<()> {
    println!("{:#?}", args);

    Ok(())
}
