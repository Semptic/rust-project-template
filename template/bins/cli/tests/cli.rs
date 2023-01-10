use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn hello_world() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("{{project-name}}")?;

    cmd.arg("world");
    cmd.assert().failure().stderr(
        predicate::str::contains("Failed to say hello")
            .and(predicate::str::contains("I will not say 'Hello, world.'!")),
    );

    Ok(())
}

#[test]
fn hello_john() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("{{project-name}}")?;

    cmd.arg("John");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello, John."));

    Ok(())
}

#[test]
fn test_add() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("{{project-name}}")?;

    cmd.args(["-vv", "John"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello, John."))
        .stderr(predicate::str::contains("1 + 1 = 2"));

    Ok(())
}
