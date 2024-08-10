use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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

#[derive(Debug, PartialEq)]
struct FileInfo {
    line_count: usize,
    word_count: usize,
    byte_count: usize,
    char_count: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }

    Ok(())
}

fn run(mut args: Args) -> Result<()> {
    // Check if all the flags are false
    let are_all_flags_false = [args.words, args.bytes, args.chars, args.lines]
        // Create an iterator.
        .iter()
        // Compare to &false because the values are references.
        .all(|v| v == &false);

    // Assign default settings if all flags are false.
    if are_all_flags_false {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    for filename in &args.files {
        match open_input_source(filename) {
            Err(e) => {
                eprintln!("{filename}: {e}")
            }
            Ok(filehandle) => {
                let file_info = get_file_info(filehandle)?;

                // Format the values into a right-justified field eight characters wide.
                println!(
                    "{:>8}{:>8}{:>8} {}",
                    file_info.line_count,
                    file_info.word_count,
                    file_info.byte_count,
                    filename
                );
            }
        }
    }

    Ok(())
}

// Accepts a filename and returns either an error or a boxed value that implements the BufRead
// trait.
// - The return type includes the dyn keyword to say that the return type's trait is dynamically
// dispatched. This allows us to abstract the idea of the input source.
// - The return type is placed into a Box. which is a way to store a value on the heap. The
// compiler does not have enough information from dyn BufRead to know the size of the return type.
// If a variable does not have a fixed known size, then Rust cannot store it on the stack. The
// solution is to instead allocate memory on the heap by putting the return value into a Box, which
// is a pointer with a known size.
fn open_input_source(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn get_file_info(mut filehandle: impl BufRead) -> Result<FileInfo> {
    // Initialize counters.
    let mut line_count = 0;
    let mut word_count = 0;
    let mut byte_count = 0;
    let mut char_count = 0;

    // Create a mutable buffer to hold each line of text.
    let mut line_buffer = String::new();

    // Create an infinite loop for reading each line from the filehandle.
    loop {
        // BufRead::read_line preserves the line endings, as opposed to BufRead::lines removing the
        // line endings.
        let bytes_read = filehandle.read_line(&mut line_buffer)?;

        // Break out of the loop when end of file has been reached.
        if bytes_read == 0 {
            break;
        }

        byte_count += bytes_read;
        line_count += 1;
        word_count += line_buffer.split_whitespace().count();
        char_count += line_buffer.chars().count();

        // Clear the line buffer for the next line of text.
        line_buffer.clear();
    }

    Ok(FileInfo {
        line_count,
        word_count,
        byte_count,
        char_count,
    })
}

// Unit tests
//
// The cfg(test) enables conditional compilation, so this module will be compiled only when
// testing.
#[cfg(test)]
mod tests {
    // Import from the parent module super (next above).
    use super::*;

    #[test]
    fn test_get_file_info() {
        // Fake a filehandle.
        let filehandle =
            std::io::Cursor::new("I don't want the world.\nI just want your half.\r\n");

        let file_info = get_file_info(filehandle);
        assert!(file_info.is_ok());

        // This comparison required FileInfo to implement the PartialEq trait.
        assert_eq!(
            file_info.unwrap(),
            FileInfo {
                line_count: 2,
                word_count: 10,
                char_count: 48,
                byte_count: 48,
            }
        );
    }
}
