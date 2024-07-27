use anyhow::{Ok, Result};
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn dies_no_args() -> Result<()> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));

    Ok(())
}

fn assert_echor(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;

    let mut cmd = Command::cargo_bin("echor")?;
    let output = cmd.args(args).output()?;
    let stdout = String::from_utf8(output.stdout)?;

    assert_eq!(stdout, expected);

    Ok(())
}

#[test]
fn hello1() -> Result<()> {
    assert_echor(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> Result<()> {
    assert_echor(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> Result<()> {
    assert_echor(
        &["Hello  there", "-n"],
        "tests/expected/hello1--no-newline.txt",
    )
}

#[test]
fn hello2_no_newline() -> Result<()> {
    assert_echor(
        &["-n", "Hello", "there"],
        "tests/expected/hello2--no-newline.txt",
    )
}
