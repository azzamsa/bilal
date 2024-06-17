use std::fs;
use std::path::{Path, PathBuf};

use miette::{NamedSource, Result, SourceOffset};
use serde::Deserialize;

use crate::error::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub latitude: f32,
    pub longitude: f32,
    pub madhab: String,
    pub method: String,
    /// Time format. 24 Hour or 12 Hour format.
    pub time_format: Option<String>,
}

/// Return a configuration struct
pub fn read() -> Result<Config, Error> {
    let config_path = &path()?;
    let file_content = fs::read_to_string(config_path).map_err(|_| Error::ConfigNotFound {
        path: config_path.to_path_buf(),
    })?;
    deserialize(&file_content)
}

/// Convert config string into a struct
fn deserialize(content: &str) -> Result<Config, Error> {
    match toml::from_str(content) {
        Ok(config) => Ok(config),
        Err(e) => {
            let range = &e.span().unwrap_or(std::ops::Range { start: 0, end: 0 });
            Err(Error::InvalidConfig {
                src: NamedSource::new("bilal.toml", content.to_owned()),
                bad_bit: SourceOffset::from_location(content, range.start, range.end),
                message: e.to_string(),
            })?
        }
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
    Ok(path)
}
