use assert_cmd::Command;
use predicates::str::contains;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn runs_help() -> TestResult {
    let mut cmd = Command::cargo_bin("xecho")?;
    cmd.arg("-h").assert().success().stdout(contains("Usage"));
    cmd.assert().success();
    Ok(())
}

#[test]
fn runs_one_string() -> TestResult {
    let mut cmd = Command::cargo_bin("xecho")?;
    cmd.arg("Hello World")
        .assert()
        .success()
        .stdout(contains("Hello World"));
    Ok(())
}

#[test]
fn runs_multiple_strings() -> TestResult {
    let mut cmd = Command::cargo_bin("xecho")?;
    cmd.args(vec!["Hello", "World", "!!!"])
        .assert()
        .success()
        .stdout(contains("Hello World !!!"));
    Ok(())
}

#[test]
fn dies_no_arg() -> TestResult {
    let mut cmd = Command::cargo_bin("xecho")?;
    cmd.assert().failure().stderr(contains("Usage"));
    Ok(())
}

#[test]
fn dies_wrong_arg() -> TestResult {
    let mut cmd = Command::cargo_bin("xecho")?;
    cmd.args(vec!["-x", "Hello World"])
        .assert()
        .failure()
        .stderr(contains("unexpected argument"));
    Ok(())
}
