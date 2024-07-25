use assert_cmd::Command;

#[test]
fn runs() {
    // Just call Result::unwrap because the binary should always be found.
    let mut cmd: Command = Command::cargo_bin("hello").unwrap();

    // Verify the command succeeds
    cmd.assert().success();
}
