use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::{env, process::Command};

#[test]
fn help() -> Result<()> {
    let mut cmd = Command::cargo_bin("bilal")?;
    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Bilal [A CLI salah time]"));
    Ok(())
}

#[test]
fn all() -> Result<()> {
    env::set_var("HOME", "./tests");
    env::set_var("APPDATA", "./tests");

    let mut cmd = Command::cargo_bin("bilal")?;
    cmd.arg("all");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Fajr"));
    Ok(())
}

#[test]
fn current() -> Result<()> {
    env::set_var("HOME", "./tests");
    env::set_var("APPDATA", "./tests");

    let mut cmd = Command::cargo_bin("bilal")?;
    cmd.arg("current").arg("--json");
    // \u{23fa} : ⏺
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\u{23fa}"));
    Ok(())
}

#[test]
fn next() -> Result<()> {
    env::set_var("HOME", "./tests");
    env::set_var("APPDATA", "./tests");

    let mut cmd = Command::cargo_bin("bilal")?;
    cmd.arg("next").arg("--json");
    // \u{25b6} : ▶
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\u{25b6}"));
    Ok(())
}
