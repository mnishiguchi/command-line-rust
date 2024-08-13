use clap::Parser;
use walkdir::WalkDir;

/// Search for files in a directory hierarchy.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
struct Args {
    /// Search path(s)
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<String>,

    /// Name(s)
    #[arg(
        short = 'n',
        long = "name",
        value_name = "NAME",
        value_parser = regex::Regex::new,
        action = clap::ArgAction::Append,
        num_args = 0..,
    )]
    names: Vec<regex::Regex>,

    /// Entry type(s)
    #[arg(
        short = 't',
        long = "type",
        value_name = "TYPE",
        value_parser = clap::value_parser!(EntryType),
        action = clap::ArgAction::Append,
        num_args = 0..,
    )]
    entry_types: Vec<EntryType>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl clap::ValueEnum for EntryType {
    // Returns the allowed variants.
    fn value_variants<'a>() -> &'a [Self] {
        &[EntryType::Dir, EntryType::File, EntryType::Link]
    }

    // Converts an enum variant to its string representation.
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            EntryType::Dir => clap::builder::PossibleValue::new("d"),
            EntryType::File => clap::builder::PossibleValue::new("f"),
            EntryType::Link => clap::builder::PossibleValue::new("l"),
        })
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if let Err(e) = do_run(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }

    Ok(())
}

fn do_run(args: Args) -> anyhow::Result<()> {
    for path in args.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => {
                    // Skip bad directories by not propagating errors.
                    eprintln!("{e}");
                }
                Ok(entry) => {
                    println!("{}", entry.path().display());
                }
            }
        }
    }

    Ok(())
}
