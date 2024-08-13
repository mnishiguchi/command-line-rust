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
    let type_filter = |walkdir_entry: &walkdir::DirEntry| -> bool {
        args.entry_types.is_empty()
            || args.entry_types.iter().any(|entry_type| match entry_type {
                EntryType::Link => walkdir_entry.file_type().is_symlink(),
                EntryType::Dir => walkdir_entry.file_type().is_dir(),
                EntryType::File => walkdir_entry.file_type().is_file(),
            })
    };

    let name_filter = |walkdir_entry: &walkdir::DirEntry| -> bool {
        args.names.is_empty()
            || args
                .names
                .iter()
                .any(|name_regex| name_regex.is_match(&walkdir_entry.file_name().to_string_lossy()))
    };

    for path in args.paths {
        let filtered_entries: Vec<_> = WalkDir::new(path)
            .into_iter()
            .filter_map(
                |walkdir_entry: Result<walkdir::DirEntry, _>| match walkdir_entry {
                    Err(e) => {
                        // Skip bad directories by not propagating errors.
                        eprintln!("{e}");
                        None
                    }
                    Ok(walkdir_entry) => {
                        // Keep this entry in the list.
                        Some(walkdir_entry)
                    }
                },
            )
            .filter(type_filter)
            .filter(name_filter)
            .map(|walkdir_entry| walkdir_entry.path().display().to_string())
            .collect();

        println!("{}", filtered_entries.join("\n"));
    }

    Ok(())
}
