use std::fs;
use std::path::{Path, PathBuf};

use crate::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub timezone: i32,
    pub latitude: f32,
    pub longitude: f32,
    pub madhab: String,
    pub method: String,
}

/// Return a configuration struct
pub fn read() -> Result<Config, Error> {
    let file_content = fs::read_to_string(path()?)?;
    deserialize(&file_content)
}

/// Convert config string into a struct
fn deserialize(content: &str) -> Result<Config, Error> {
    match toml::from_str(content) {
        Ok(config) => Ok(config),
        Err(e) => Err(Error::InvalidConfig {
            message: e.to_string(),
        }),
    }
}

/// Return configuration path
fn path() -> Result<PathBuf, Error> {
    let path = if cfg!(windows) {
        Path::new(&std::env::var("APPDATA")?)
            .join("Bilal")
            .join("config.toml")
    } else {
        Path::new(&std::env::var("HOME")?)
            .join(".config")
            .join("bilal")
            .join("config.toml")
    };

    if path.exists() {
        Ok(path)
    } else {
        Err(Error::ConfigNotFound { path })
    }
}
