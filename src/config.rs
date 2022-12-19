use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::Error;

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
        Err(Error::NotFound("configuration file is not found".into()))
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
fn deserialize(content: &str) -> Result<Config, Error> {
    toml::from_str(content).map_err(|e| Error::InvalidConfig { source: e })
}

/// Return a configuration struct
pub fn get() -> Result<Config, Error> {
    let file_content = fs::read_to_string(path()?)?;
    deserialize(&file_content)
}
