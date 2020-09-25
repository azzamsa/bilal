use std::fs;
use std::path::Path;

use serde::Deserialize;
use toml::de;

#[derive(Deserialize)]
pub struct Config {
    pub latitude: f64,
    pub longitude: f64,
}

/// Read a config file.
fn read_config_file() -> String {
    let home = std::env::var("HOME").unwrap();
    let config_path = format!("{}/.config/bilal/config.toml", home).to_string();
    if !Path::new(&config_path).exists() {
        eprintln!("Can't find config file. Please Create one.");
        std::process::exit(1);
    }
    fs::read_to_string(config_path).expect("Can't read file")
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
