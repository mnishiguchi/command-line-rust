use clap::Parser;
use regex::{Regex, RegexBuilder};
use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    mem,
};
use walkdir::WalkDir;

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
        // RegexBuilder::build rejects any pattern that is not a valid regular expression. There
        // are many syntaxes for writing regular expressions.
        .build()
        // If build returns an error, create an error message stating that the given pattern is
        // invalid.
        .map_err(|_| anyhow::anyhow!(r#"Invalid pattern "{}""#, args.pattern))?;

    println!(r#"pattern "{pattern}""#);

    let entries = find_files(&args.files, args.recursive);

    for entry in entries {
        match entry {
            Err(e) => {
                eprintln!("{e}")
            }
            Ok(filename) => {
                match open_input_file(&filename) {
                    Err(e) => {
                        eprintln!("{filename}: {e}")
                    }
                    Ok(filehandle) => {
                        let matches = find_lines(filehandle, &pattern, args.invert_match);
                        println!("Found {matches:?}");
                    }
                }
                println!(r#"file "{filename}""#)
            }
        }
    }

    Ok(())
}

// Opening user-provided input source

fn open_input_file(filename: &str) -> anyhow::Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        path => Ok(Box::new(BufReader::new(File::open(path)?))),
    }
}

fn find_files(paths: &[String], recursive: bool) -> Vec<anyhow::Result<String>> {
    // Initialize an empty vector to hold the results.
    let mut results = vec![];

    // Iterate over each of the given paths.
    for path in paths {
        match path.as_str() {
            // First, accept a dash (-) as a path for STDIN.
            "-" => {
                results.push(Ok(path.to_string()));
            }
            _ => {
                // Try to get the path's metadata.
                match fs::metadata(path) {
                    Ok(metadata) => {
                        if metadata.is_dir() {
                            if recursive {
                                // Add to the results all the files in the given directory.
                                for entry in WalkDir::new(path)
                                    .into_iter()
                                    // Iterator::flatten will take the Ok or Some variants for
                                    // Result and Option types and will ignore Err and None
                                    // variants, meaning it will ignore any errors with files
                                    // found by recursing through directories.
                                    .flatten()
                                    .filter(|e| e.file_type().is_file())
                                {
                                    results.push(Ok(entry.path().display().to_string()));
                                }
                            } else {
                                results.push(Err(anyhow::anyhow!("{path} is a directory")));
                            }
                        } else if metadata.is_file() {
                            // Add the file to the results.
                            results.push(Ok(path.to_string()));
                        }
                    }
                    Err(e) => {
                        // Nonexistent files.
                        results.push(Err(anyhow::anyhow!("{path}: {e}")));
                    }
                }
            }
        }
    }

    results
}

fn find_lines(
    mut filehandle: impl BufRead,
    pattern: &Regex,
    invert_match: bool,
) -> anyhow::Result<Vec<String>> {
    let mut matches = vec![];
    let mut line = String::new();

    loop {
        let bytes = filehandle.read_line(&mut line)?;

        if bytes == 0 {
            break;
        }

        // The bitwise XOR comparison (^) determines if the line should be included.
        if pattern.is_match(&line) ^ invert_match {
            // Use std::mem::take to take ownership of the line.
            // Alternatively, we sould clone to copy the string.
            matches.push(mem::take(&mut line));
        }

        line.clear();
    }

    Ok(matches)
}

// Unit testing

#[cfg(test)]
mod tests {
    use super::{find_files, find_lines};
    use rand::{distributions::Alphanumeric, Rng};
    use regex::{Regex, RegexBuilder};
    use std::io::Cursor;

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let files = find_files(&["./tests/inputs/fox.txt".to_string()], false);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].as_ref().unwrap(), "./tests/inputs/fox.txt");

        // The function should reject a directory without the recursive option
        let files = find_files(&["./tests/inputs".to_string()], false);
        assert_eq!(files.len(), 1);
        if let Err(e) = &files[0] {
            assert_eq!(e.to_string(), "./tests/inputs is a directory");
        }

        // Verify that the function recurses to find four files in the directory
        let res = find_files(&["./tests/inputs".to_string()], true);
        let mut files: Vec<String> = res
            .iter()
            .map(|r| r.as_ref().unwrap().replace("\\", "/"))
            .collect();
        files.sort();
        assert_eq!(files.len(), 4);
        assert_eq!(
            files,
            vec![
                "./tests/inputs/bustle.txt",
                "./tests/inputs/empty.txt",
                "./tests/inputs/fox.txt",
                "./tests/inputs/nobody.txt",
            ]
        );

        // Generate a random string to represent a nonexistent file
        let bad: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        // Verify that the function returns the bad file as an error
        let files = find_files(&[bad], false);
        assert_eq!(files.len(), 1);
        assert!(files[0].is_err());
    }

    #[test]
    fn test_find_lines() {
        let text = b"Lorem\nIpsum\r\nDOLOR";

        // The pattern "or" should match the one line "Lorem"
        let re1 = Regex::new("or").unwrap();
        let matches = find_lines(Cursor::new(&text), &re1, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);

        // When interted, the function should match the other two lines
        let matches = find_lines(Cursor::new(&text), &re1, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // This regex will be case-insensitive
        let re2 = RegexBuilder::new("or")
            .case_insensitive(true)
            .build()
            .unwrap();

        // The two lines "Lorem" and "DOLOR" should match
        let matches = find_lines(Cursor::new(&text), &re2, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // When inverted, the one remaining line should match
        let matches = find_lines(Cursor::new(&text), &re2, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);
    }
}
