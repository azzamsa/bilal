use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::{env, process::Command};

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
    env::set_var("HOME", "./tests");
    let mut cmd = Command::cargo_bin("bilal").unwrap();
    cmd.arg("all");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Fajr"));
}

#[test]
fn current() {
    env::set_var("HOME", "./tests");
    let mut cmd = Command::cargo_bin("bilal").unwrap();
    cmd.arg("current").arg("--json");
    // \u{23fa} : ⏺
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\u{23fa}"));
}

#[test]
fn next() {
    env::set_var("HOME", "./tests");
    let mut cmd = Command::cargo_bin("bilal").unwrap();
    cmd.arg("next").arg("--json");
    // \u{25b6} : ▶
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\u{25b6}"));
}
