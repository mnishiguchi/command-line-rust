struct Cli {
    pattern: String,
    // PathBuf is like a String but for file system paths that work cross-platform.
    path: std::path::PathBuf,
}

fn main() {
    // Example
    //   $ cargo run --quiet -- some-pattern some-file
    //   pattern: "some-pattern", path: "some-file"
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    let args = Cli {
        pattern,
        path: std::path::PathBuf::from(path),
    };

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
