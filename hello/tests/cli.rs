use assert_cmd::Command;
use pretty_assertions::assert_eq;

#[test]
fn runs() {
    // Just call Result::unwrap because the binary should always be found.
    let mut cmd: Command = Command::cargo_bin("hello").unwrap();

    // Execute the command.
    let output = cmd.output().expect("fail");

    // Verify the command succeeds.
    assert!(output.status.success());

    // Convert the output of the program to UTF-8.
    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");

    // Verify the output is correct.
    assert_eq!(stdout, "Hello, world!\n");
}
