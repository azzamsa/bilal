use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use toml::de;

#[derive(Deserialize)]
pub struct Config {
    pub latitude: f64,
    pub longitude: f64,
}

/// Read a config file.
fn read_config_file() -> String {
    let _config_path: PathBuf = {
        if cfg!(windows) {
            Path::new(&std::env::var("APPDATA").unwrap())
                .join("Azzamsa")
                .join("Bilal")
                .join("config.toml")
        } else {
            Path::new(&std::env::var("HOME").unwrap())
                .join(".config")
                .join("bilal")
                .join("config.toml")
        }
    };

    if !_config_path.exists() {
        eprintln!("Can't find config file. Please Create one.");
        std::process::exit(1);
    }
    fs::read_to_string(_config_path).expect("Can't read file")
}

// Deserialize config string intro struct.
fn deserialize_config(config: &str) -> Result<Config, de::Error> {
    let config: Config = toml::from_str(&config)?;
    Ok(config)
}

/// Return a config key, value
pub fn get_config() -> Config {
    let config_content = read_config_file();
    let config = deserialize_config(&config_content);

    match config {
        Ok(conf) => conf,
        Err(_) => {
            eprintln!("Invalid config file.");
            std::process::exit(1);
        }
    }
}
