use clap::Parser;
use regex::Regex;
use std::{
    borrow::Cow,
    fs::File,
    io::{self, BufRead, BufReader},
    num::NonZeroUsize,
    ops::Range,
};

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

// Represents spans of positive integer values.
type PositionList = Vec<Range<usize>>;

// Represents the variants for extracting fields, bytes or characters.
#[derive(Debug)]
pub enum SelectionMode {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

fn main() {
    let args = CliArguments::parse();

    if let Err(e) = do_run(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn do_run(args: CliArguments) -> anyhow::Result<()> {
    // Break the delimiter string into a vector of u8.
    let delimiter_bytes: &[u8] = args.delimiter.as_bytes();

    if delimiter_bytes.len() != 1 {
        // Use a raw string so the contained double quotes do not require excaping.
        anyhow::bail!(r#"--delim "{}" must be a single byte"#, args.delimiter);
    }

    // Get the first byte. It is safe to call Option::unwrap because we have verified that this
    // vector has exactly one byte.
    let delimiter_byte: Option<&u8> = delimiter_bytes.first();
    let delimiter_byte: &u8 = delimiter_byte.unwrap();
    let delimiter_byte: u8 = *delimiter_byte;

    let parsed_position_lists = (
        args.selection_arguments
            .fields
            .map(parse_position)
            .transpose()?,
        args.selection_arguments
            .bytes
            .map(parse_position)
            .transpose()?,
        args.selection_arguments
            .chars
            .map(parse_position)
            .transpose()?,
    );

    let selection_mode: SelectionMode = match parsed_position_lists {
        (Some(position_list), _, _) => SelectionMode::Fields(position_list),
        (_, Some(position_list), _) => SelectionMode::Bytes(position_list),
        (_, _, Some(position_list)) => SelectionMode::Chars(position_list),
        _ => unreachable!("Must have --fields, --bytes, or --chars"),
    };

    for filename in &args.files {
        match (open_input_file(filename), &selection_mode) {
            (Err(e), _) => {
                // Skips bad files.
                eprintln!("{}: {}", filename, e);
            }
            (Ok(filehandle), SelectionMode::Fields(position_list)) => {
                print_selected_fields(filehandle, position_list, delimiter_byte)?
            }
            (Ok(filehandle), SelectionMode::Bytes(position_list)) => {
                print_selected_bytes(filehandle, position_list)?
            }
            (Ok(filehandle), SelectionMode::Chars(position_list)) => {
                print_selected_chars(filehandle, position_list)?
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

// Parsing user-provided position text

/// Parses comma-delimited position entries. The entry can be either single digit or hyphenated
/// range.
fn parse_position(position_text: String) -> anyhow::Result<PositionList> {
    position_text
        .split(',')
        .into_iter()
        .map(|value| match parse_single_digit_position(value) {
            Ok(parsed) => Ok(parsed),
            Err(_) => match parse_hyphenated_position(value) {
                Ok(parsed) => Ok(parsed),
                Err(e) => Err(e),
            },
        })
        .collect()
}

fn parse_single_digit_position(value: &str) -> anyhow::Result<Range<usize>> {
    let single_digit_regex = Regex::new(r"^(\d+)$").unwrap();

    match single_digit_regex.captures(value) {
        Some(captures) => {
            let n: &str = &captures[0];
            let n: usize = parse_index(n)?;

            Ok(n..n + 1)
        }
        None => anyhow::bail!(r#"illegal list value: "{}""#, value),
    }
}

fn parse_hyphenated_position(value: &str) -> anyhow::Result<Range<usize>> {
    let range_regex = Regex::new(r"^(\d+)-(\d+)$").unwrap();

    match range_regex.captures(value) {
        Some(captures) => {
            let n1 = parse_index(&captures[1])?;
            let n2 = parse_index(&captures[2])?;

            if n1 >= n2 {
                anyhow::bail!(
                    "First number in range ({}) must be lower than second number ({})",
                    n1 + 1,
                    n2 + 1,
                );
            }

            Ok(n1..n2 + 1)
        }
        None => anyhow::bail!(r#"illegal list value: "{}""#, value),
    }
}

/// Parses a string into a positive index value one less than the given number.
///
/// The given string may not start with a plus sign, and the parsed value must be greater than
/// zero.
fn parse_index(index_text: &str) -> anyhow::Result<usize> {
    // Create a closure that formats an error string.
    let error_message =
        || -> anyhow::Error { anyhow::anyhow!(r#"illegal list value: "{}""#, index_text) };

    // Check if the input value starts with a plus sign.
    if index_text.starts_with('+') {
        // Return early with an error.
        anyhow::bail!(error_message());
    }

    // Parse the input text, indicating the return type of std::num::NonZeroUsize (aka positive
    // integer).
    match index_text.parse::<NonZeroUsize>() {
        Ok(value) => {
            // Cast the value from NonZeroUsize to a usize.
            let value: NonZeroUsize = value;
            let value: usize = usize::from(value);

            // Decrement the value to a zero based offset.
            Ok(value - 1)
        }
        Err(_) => Err(error_message()),
    }
}

// Extracting selected part from a line

fn extract_fields_from_record(
    record: &csv::StringRecord,
    position_list: &[Range<usize>],
) -> Vec<String> {
    // There is another way to write this function so that it will return a Vec<&str>, which will be
    // slightly more memory efficient as it won't make copies of strings. The trade off is that we
    // must indicate the lifetimes.
    position_list
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| record.get(i)))
        .map(String::from)
        .collect()
}

fn extract_bytes_from_line(line: &str, position_list: &[Range<usize>]) -> String {
    let bytes: &[u8] = line.as_bytes();

    // We use std::iter::Copied to create copies of the elements. The reason is that Iterator::get
    // returns a vector of byte references (&Vec<&u8>), but String::from_utf8_lossy expects a slice
    // of bytes (&[u8]).
    let selected: Vec<u8> = position_list
        .iter()
        .cloned()
        // Select the bytes for each range in the position list.
        .flat_map(|range| range.filter_map(|i| bytes.get(i)).copied())
        .collect();

    // Create a possibly invalid UTF-8 string from bytes.
    let selected: Cow<str> = String::from_utf8_lossy(&selected);

    // Clone the data as needed.
    let selected: String = selected.into_owned();

    selected
}

fn extract_chars_from_line(line: &str, position_list: &[Range<usize>]) -> String {
    let chars: Vec<char> = line.chars().collect();

    position_list
        .iter()
        .cloned()
        // Select the characters for each range in the position list.
        .flat_map(|range| range.filter_map(|i| chars.get(i)))
        .collect()
}

// Printing selected part of the file

fn print_selected_fields(
    filehandle: Box<dyn BufRead>,
    position_list: &[Range<usize>],
    delimiter_byte: u8,
) -> anyhow::Result<()> {
    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(delimiter_byte)
        .has_headers(false)
        .from_reader(filehandle);

    let mut csv_writer = csv::WriterBuilder::new()
        .delimiter(delimiter_byte)
        .from_writer(io::stdout());

    for record in csv_reader.records() {
        let record: csv::StringRecord = record?;
        csv_writer.write_record(extract_fields_from_record(&record, position_list))?;
    }

    Ok(())
}

fn print_selected_bytes(
    filehandle: Box<dyn BufRead>,
    position_list: &[Range<usize>],
) -> anyhow::Result<()> {
    for line in filehandle.lines() {
        let line: &str = &line?;
        println!("{}", extract_bytes_from_line(&line, position_list));
    }

    Ok(())
}

fn print_selected_chars(
    filehandle: Box<dyn BufRead>,
    position_list: &[Range<usize>],
) -> anyhow::Result<()> {
    for line in filehandle.lines() {
        let line: &str = &line?;
        println!("{}", extract_chars_from_line(&line, position_list));
    }

    Ok(())
}

// Unit testing

#[cfg(test)]
mod unit_tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_parse_position() {
        // The empty string is an error.
        assert!(parse_position("".to_string()).is_err());

        // Zero is an error.
        let result = parse_position("0".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            r#"illegal list value: "0""#
        );

        let result = parse_position("0-1".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            r#"illegal list value: "0""#
        );

        // A leading "+" is an error.
        let result = parse_position("+1".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            r#"illegal list value: "+1""#
        );

        let result = parse_position("+1-2".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            r#"illegal list value: "+1-2""#
        );

        let result = parse_position("1-+2".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            r#"illegal list value: "1-+2""#
        );

        // Any non-number is an error.
        let result = parse_position("a".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            r#"illegal list value: "a""#
        );

        let result = parse_position("1,a".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            r#"illegal list value: "a""#
        );

        let result = parse_position("1-a".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            r#"illegal list value: "1-a""#
        );

        let result = parse_position("a-1".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            r#"illegal list value: "a-1""#
        );

        // Improper ranges
        assert!(parse_position("-".to_string()).is_err());
        assert!(parse_position(",".to_string()).is_err());
        assert!(parse_position("1,".to_string()).is_err());
        assert!(parse_position("1-".to_string()).is_err());
        assert!(parse_position("1-1-1".to_string()).is_err());
        assert!(parse_position("1-1-a".to_string()).is_err());

        // First number must be less than the second
        let result = parse_position("1-1".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            r#"First number in range (1) must be lower than second number (1)"#
        );

        let result = parse_position("2-1".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            r#"First number in range (2) must be lower than second number (1)"#
        );

        // Accepable ranges
        let result = parse_position("1".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![0..1]);

        let result = parse_position("1".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![0..1]);

        let result = parse_position("01".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![0..1]);

        let result = parse_position("1,3".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![0..1, 2..3]);

        let result = parse_position("001,003".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![0..1, 2..3]);

        let result = parse_position("1-3".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![0..3]);

        let result = parse_position("0001-03".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![0..3]);

        let result = parse_position("1,7,3-5".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![0..1, 6..7, 2..5]);

        let result = parse_position("15,19-20".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![14..15, 18..20]);
    }

    #[test]
    fn test_extract_fields() {
        let rec = csv::StringRecord::from(vec!["Captain", "Sham", "12345"]);
        assert_eq!(extract_fields_from_record(&rec, &[0..1]), &["Captain"]);
        assert_eq!(extract_fields_from_record(&rec, &[1..2]), &["Sham"]);
        assert_eq!(
            extract_fields_from_record(&rec, &[0..1, 2..3]),
            &["Captain", "12345"]
        );
        assert_eq!(
            extract_fields_from_record(&rec, &[0..1, 3..4]),
            &["Captain"]
        );
        assert_eq!(
            extract_fields_from_record(&rec, &[1..2, 0..1]),
            &["Sham", "Captain"]
        );
    }

    #[test]
    fn test_extract_chars() {
        assert_eq!(extract_chars_from_line("", &[0..1]), "".to_string());
        assert_eq!(extract_chars_from_line("ábc", &[0..1]), "á".to_string());
        assert_eq!(
            extract_chars_from_line("ábc", &[0..1, 2..3]),
            "ác".to_string()
        );
        assert_eq!(extract_chars_from_line("ábc", &[0..3]), "ábc".to_string());
        assert_eq!(
            extract_chars_from_line("ábc", &[2..3, 1..2]),
            "cb".to_string()
        );
        assert_eq!(
            extract_chars_from_line("ábc", &[0..1, 1..2, 4..5]),
            "áb".to_string()
        );
    }

    #[test]
    fn test_extract_bytes() {
        assert_eq!(extract_bytes_from_line("ábc", &[0..1]), "�".to_string());
        assert_eq!(extract_bytes_from_line("ábc", &[0..2]), "á".to_string());
        assert_eq!(extract_bytes_from_line("ábc", &[0..3]), "áb".to_string());
        assert_eq!(extract_bytes_from_line("ábc", &[0..4]), "ábc".to_string());
        assert_eq!(
            extract_bytes_from_line("ábc", &[3..4, 2..3]),
            "cb".to_string()
        );
        assert_eq!(
            extract_bytes_from_line("ábc", &[0..2, 5..6]),
            "á".to_string()
        );
    }
}
