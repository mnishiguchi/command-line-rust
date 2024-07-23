fn main() {
    // Example
    //   $ cargo run --quiet -- some-pattern some-file
    //   pattern: "some-pattern", path: "some-file"
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    println!("pattern: {:?}, path: {:?}", pattern, path);
}
