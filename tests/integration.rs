use std::{env, error::Error, process::Command};

use assert_cmd::{crate_name, prelude::*};
use assert_fs::{prelude::*, TempDir};
use predicates::prelude::*;

#[test]
fn help() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Bilal [A CLI salah time]"));
    Ok(())
}

#[test]
fn all() -> Result<(), Box<dyn Error>> {
    let temp_dir = setup_config()?;
    env::set_var(
        "BILAL_CONFIG",
        format!("{}/config.toml", temp_dir.path().display()),
    );

    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("all");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Fajr"));
    Ok(())
}

#[test]
fn current() -> Result<(), Box<dyn Error>> {
    let temp_dir = setup_config()?;
    env::set_var(
        "BILAL_CONFIG",
        format!("{}/config.toml", temp_dir.path().display()),
    );

    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("current").arg("--json");
    // \u{23fa} : ⏺
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\u{23fa}"));
    Ok(())
}

#[test]
fn next() -> Result<(), Box<dyn Error>> {
    let temp_dir = setup_config()?;
    env::set_var(
        "BILAL_CONFIG",
        format!("{}/config.toml", temp_dir.path().display()),
    );

    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("next").arg("--json");
    // \u{25b6} : ▶
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\u{25b6}"));
    Ok(())
}

fn setup_config() -> Result<TempDir, Box<dyn Error>> {
    let temp_dir = assert_fs::TempDir::new()?;
    let config = temp_dir.child("config.toml");
    config.write_str(&config_base())?;
    Ok(temp_dir)
}

fn config_base() -> String {
    let content = r#"
latitude = -6.18233995
longitude = 106.84287154
madhab = "Shafi"
method = "Egyptian"
"#;
    content.to_string()
}
