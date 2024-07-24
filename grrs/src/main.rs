use clap::Parser;

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    // PathBuf is like a String but for file system paths that work cross-platform.
    path: std::path::PathBuf,
}

fn main() {
    // Example
    //   $ cargo run --quiet -- some-pattern some-file
    //   pattern: "some-pattern", path: "some-file"

    // Cli::parse() is meant to be used in our main(); don't use it in other places.
    let args = Cli::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
