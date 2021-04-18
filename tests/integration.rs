use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn help() {
    let mut cmd = Command::cargo_bin("bilal").unwrap();
    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Bilal [A CLI salah time]"));
}

#[test]
fn all() {
    let mut cmd = Command::cargo_bin("bilal").unwrap();
    cmd.arg("--all");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Fajr"));
}
