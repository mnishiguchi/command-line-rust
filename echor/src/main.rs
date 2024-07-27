use clap::Arg;
use clap::ArgAction;
use clap::Command;

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Masatoshi N")
        .about("Rust version of `echo`")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not print newline"),
        )
        .get_matches();

    // The type annotation is required because Iterator::collect can return many diffrent types.
    let text: Vec<String> = matches.get_many("text").unwrap().cloned().collect();

    let omit_newline = matches.get_flag("omit_newline");
    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{}", text.join(" "), ending);

    ()
}
