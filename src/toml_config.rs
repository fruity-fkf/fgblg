use serde::Deserialize;
use std::{fs, io};
use toml::from_str;
//NOTE: turns out you're supposed ot use snake_case for mod files. That's a bit silly IMO
#[derive(Debug, Deserialize)]
pub struct Config {
    pub theme: String,
    pub template: String,
    pub home_template: String,
    pub code_theme: String,
}

pub fn read_config(path: &str) -> io::Result<Config> {
    let contents = fs::read_to_string(path)?;
    let config: Config = from_str(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    Ok(config)
}
