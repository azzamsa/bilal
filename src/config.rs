use std::fs;
use std::path::{Path, PathBuf};

use miette::{NamedSource, Result, SourceOffset};
use serde::Deserialize;

use crate::error::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub latitude: f32,
    pub longitude: f32,
    pub madhab: String,
    pub method: String,
    /// Time format preference.
    /// "24H" for 24-hour format or "12H" for 12-hour format with AM/PM.
    #[serde(default = "default_time_format")]
    #[serde(deserialize_with = "deserialize_time_format")]
    pub time_format: TimeFormat,
    /// Available languages: en, id
    #[serde(default = "default_locale")]
    pub locale: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum TimeFormat {
    #[serde(rename = "24H")]
    H24,
    #[serde(rename = "12H")]
    H12,
}

/// Return a configuration struct
pub fn read(custom_path: Option<PathBuf>) -> Result<Config, Error> {
    let config_path = &path(custom_path)?;
    let file_content = fs::read_to_string(config_path).map_err(|_| Error::ConfigNotFound {
        path: config_path.clone(),
    })?;
    parse(&file_content)
}

/// Convert config string into a struct
fn parse(content: &str) -> Result<Config, Error> {
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
fn path(custom_path: Option<PathBuf>) -> Result<PathBuf, Error> {
    if let Some(path) = custom_path {
        return Ok(path);
    }

    match &std::env::var("BILAL_CONFIG") {
        Err(_) => {
            if cfg!(windows) {
                Ok(Path::new(&std::env::var("APPDATA")?)
                    .join("Bilal")
                    .join("config.toml"))
            } else {
                Ok(Path::new(&std::env::var("HOME")?)
                    .join(".config")
                    .join("bilal")
                    .join("config.toml"))
            }
        }
        Ok(path) => Ok(PathBuf::from(path)),
    }
}

fn deserialize_time_format<'de, D>(deserializer: D) -> Result<TimeFormat, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let format: String = Deserialize::deserialize(deserializer)?;
    match format.as_ref() {
        "24H" => Ok(TimeFormat::H24),
        "12H" => Ok(TimeFormat::H12),
        _ => Err(serde::de::Error::custom(
            r#"Invalid time format. Expected "24H" or "12H""#,
        )),
    }
}

fn default_time_format() -> TimeFormat {
    TimeFormat::H24
}

fn default_locale() -> String {
    "en".to_string()
}
