use clap::{ArgAction, Parser};
use std::{
    cmp::Ordering,
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

// Represents the column where the value should be printed
enum Column<'a> {
    Col1(&'a str),
    Col2(&'a str),
    Col3(&'a str),
}

fn main() {
    if let Err(e) = do_run(CliArguments::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn do_run(args: CliArguments) -> anyhow::Result<()> {
    // println!("{:#?}", args);

    let file1 = &args.file1;
    let file2 = &args.file2;

    // Prohibit that both the filenames being "-"
    if file1 == "-" && file2 == "-" {
        anyhow::bail!(r#"Both input files cannot be STDIN ("-")"#);
    }

    // Create a closure to downcase each line of text when args.insensitive is true.
    let apply_case = |line: String| {
        if args.ignore_case {
            line.to_lowercase()
        } else {
            line
        }
    };

    let print_column = |col: Column| {
        let mut output_column_values = vec![];

        match col {
            Column::Col1(text) => {
                if args.show_col1 {
                    output_column_values.push(text)
                }
            }
            Column::Col2(text) => {
                if args.show_col2 {
                    if args.show_col1 {
                        output_column_values.push(""); // fill col1 in with a spacer
                    }

                    output_column_values.push(text);
                }
            }
            Column::Col3(text) => {
                if args.show_col3 {
                    if args.show_col1 {
                        output_column_values.push(""); // fill col1 in with a spacer
                    }

                    if args.show_col2 {
                        output_column_values.push(""); // fill col2 in with a spacer
                    }

                    output_column_values.push(text);
                }
            }
        }

        if !output_column_values.is_empty() {
            println!("{}", output_column_values.join(&args.delimiter));
        }
    };

    // Attempt to open the two input files
    let filehandle1 = open_input_file(file1)?;
    let filehandle2 = open_input_file(file2)?;
    // println!(r#"Opened "{file1}" and "{file2}""#);

    // Use BufRead::lines to read files as it is not necessary to preserve line endings.
    // Create iterators, remove errors, then apply case-sensitivity to each line.
    let mut lines1 = filehandle1.lines().map_while(Result::ok).map(apply_case);
    let mut lines2 = filehandle2.lines().map_while(Result::ok).map(apply_case);

    // The Iterator::text method advances an iterator and returns the next value.
    // Here it will retrieve the first line from a filehandle.
    let mut line1 = lines1.next();
    let mut line2 = lines2.next();

    while line1.is_some() || line2.is_some() {
        // Compare all the possible combinations of the two line variables for two variants.
        match (&line1, &line2) {
            (Some(val1), Some(val2)) => {
                // Use Ord::cmp to compare the first value to the second. This will return an enum variant of
                // std::cmp::Ordering.
                match val1.cmp(val2) {
                    // When the two values are the same
                    Ordering::Equal => {
                        // print the value in column 3
                        print_column(Column::Col3(val1));

                        // get the values from each of the files
                        line1 = lines1.next();
                        line2 = lines2.next();
                    }
                    // When the first value is less than the second
                    Ordering::Less => {
                        // print the first value in column 1
                        print_column(Column::Col1(val1));

                        // get the next value from the first file
                        line1 = lines1.next();
                    }
                    // When the first value is greater than the second
                    Ordering::Greater => {
                        // print the second value in column 2
                        print_column(Column::Col2(val2));

                        // get the next value from the second file
                        line2 = lines2.next();
                    }
                }
            }
            // When there is a value only from the first file
            (Some(val1), None) => {
                // print the value in column 1
                print_column(Column::Col1(val1));

                // get the next value from the first file
                line1 = lines1.next();
            }
            // When there is a value only from the second file
            (None, Some(val2)) => {
                // print the value in column 2
                print_column(Column::Col2(val2));

                // get the next value from the second file
                line2 = lines2.next();
            }
            _ => (),
        };
    }

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
