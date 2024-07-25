use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn find_matches(f: File, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    // BufReader.lines() reads a file more efficiently than std::fs::read_to_string().
    let reader: BufReader<File> = BufReader::new(f);

    for line in reader.lines() {
        // With a question mark, Rust will internally expand the Result.
        let s = line?;

        if s.contains(&pattern) {
            // writeln!() returns an io::Result because writing can fail.
            writeln!(writer, "{}", s)?;
        }
    }

    // This means "Result is OK and has no content."
    // The last expression of any block in Rust is its return value.
    Ok(())
}
