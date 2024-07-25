use std::process::Command;

#[test]
fn runs() {
    let mut cmd: Command = Command::new("./target/debug/hello");

    // Run the command and capture the output.
    let res: Result<_, _> = cmd.output();

    // Verify the result is an OK variant.
    assert!(res.is_ok());
}
