use clap::Parser;

/// Concatenate FILE(s) to standard output.
/// With no FILE, or when FILE is -, read standard input.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number all output lines
    #[arg(short = 'n', long, conflicts_with = "number_nonblank")]
    number: bool,

    /// Number nonempty output lines
    #[arg(short = 'b', long)]
    number_nonblank: bool,
}

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);

    ()
}
