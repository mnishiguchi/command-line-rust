use clap::Parser;
use regex::RegexBuilder;

/// Print lines that patch patterns
#[derive(Debug, clap::Parser, Clone)]
#[command(author, version, about)]
struct CliArguments {
    // Positional arguments
    //
    // - The order in which positional arguments are defined is important.

    /// search pattern
    #[arg()]
    pattern: String,

    /// input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    // Optional arguments
    //
    // - The order in which optional arguments are defined does not matter.

    /// Ignore case distinctions in patterns and data
    #[arg(short, long)]
    ignore_case: bool,

    /// Recursive
    #[arg(short, long)]
    recursive: bool,

    /// Print only a count of selected lines per FILE
    #[arg(short, long)]
    count: bool,

    /// Select non-matching lines
    #[arg(short = 'v', long)]
    invert_match: bool,
}

fn main() {
    if let Err(e) = do_run(CliArguments::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn do_run(args: CliArguments) -> anyhow::Result<()> {
    // A RegexBuilder allows for non-default configuration like case-insensitive matching.
    let pattern = RegexBuilder::new(&args.pattern)
        .case_insensitive(args.ignore_case)
        .build()
        // If build returns an error, create an error message stating that the given pattern is
        // invalid.
        .map_err(|_| anyhow::anyhow!(r#"Invalid pattern "{}""#, args.pattern))?;

    println!(r#"pattern "{pattern}""#);

    Ok(())
}

