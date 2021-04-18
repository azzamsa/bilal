use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::BilalError;

/// Return configuration path
fn path() -> Result<PathBuf, BilalError> {
    let path = if cfg!(windows) {
        Path::new(&std::env::var("APPDATA").unwrap())
            .join("Bilal")
            .join("config.toml")
    } else {
        Path::new(&std::env::var("HOME").unwrap())
            .join(".config")
            .join("bilal")
            .join("config.toml")
    };

    if path.exists() {
        Ok(path)
    } else {
        Err(BilalError::NoFile(path))
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub timezone: i32,
    pub latitude: f32,
    pub longitude: f32,
    pub madhab: String,
    pub method: String,
}

/// Convert config string into a struct
fn deserialize(content: &str) -> Result<Config, BilalError> {
    toml::from_str(content).map_err(|e| BilalError::InvalidConfig { source: e })
}

/// Return a configuration struct
pub fn get() -> Result<Config, BilalError> {
    let file_content = fs::read_to_string(path()?)?;
    deserialize(&file_content)
}
