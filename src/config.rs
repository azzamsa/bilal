use std::fs;
use std::path::Path;

use std::collections::HashMap;
use toml::Value;

fn read_config_file() -> String {
    let home = std::env::var("HOME").unwrap();
    let config_path = format!("{}/.config/bilal/config.toml", home).to_string();
    if !Path::new(&config_path).exists() {
        panic!("Can't find config file")
    }
    fs::read_to_string(config_path).expect("Can't read file")
}

pub fn get_config() -> HashMap<String, f64> {
    let mut configs = HashMap::new();
    let config_content = read_config_file();

    let value = config_content.parse::<Value>().unwrap();
    configs.insert(
        "latitude".to_string(),
        value.clone()["latitude"].as_float().unwrap(),
    );
    configs.insert(
        "longitude".to_string(),
        value.clone()["longitude"].as_float().unwrap(),
    );

    return configs;
}
