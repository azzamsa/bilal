use std::{env, error::Error, process::Command};

use assert_cmd::{crate_name, prelude::*};
use assert_fs::fixture::FileWriteStr;
use assert_fs::{fixture::ChildPath, prelude::*, TempDir};
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
    let (temp_dir, config) = setup_config()?;
    config.write_str(&config_base())?;
    env::set_var("BILAL_CONFIG", config.to_path_buf());

    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("all");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Fajr"));
    // Make sure it is not 12H format
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("AM").not());

    temp_dir.close()?;
    Ok(())
}

#[test]
fn current() -> Result<(), Box<dyn Error>> {
    let (temp_dir, config) = setup_config()?;
    config.write_str(&config_base())?;
    env::set_var("BILAL_CONFIG", config.to_path_buf());

    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("current").arg("--json");
    // \u{23fa} : ⏺
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\u{23fa}"));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn next() -> Result<(), Box<dyn Error>> {
    let (temp_dir, config) = setup_config()?;
    config.write_str(&config_base())?;
    env::set_var("BILAL_CONFIG", config.to_path_buf());

    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("next").arg("--json");
    // \u{25b6} : ▶
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\u{25b6}"));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn all_12h_format() -> Result<(), Box<dyn Error>> {
    let (temp_dir, config) = setup_config()?;
    config.write_str(&config_12h())?;
    env::set_var("BILAL_CONFIG", config.to_path_buf());

    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("all");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("AM"));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn next_12h_format() -> Result<(), Box<dyn Error>> {
    let (temp_dir, config) = setup_config()?;
    config.write_str(&config_12h())?;
    env::set_var("BILAL_CONFIG", config.to_path_buf());

    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("next");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("AM").or(predicate::str::contains("PM")));

    temp_dir.close()?;
    Ok(())
}

fn setup_config() -> Result<(TempDir, ChildPath), Box<dyn Error>> {
    let temp_dir = TempDir::new()?;
    let config = temp_dir.child("config.toml");
    Ok((temp_dir, config))
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

fn config_12h() -> String {
    let content = r#"
latitude = -6.18233995
longitude = 106.84287154
madhab = "Shafi"
method = "Egyptian"
time_format = "12H"
"#;
    content.to_string()
}
