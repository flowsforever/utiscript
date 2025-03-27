use std::fs;
use std::path::Path;
use toml::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub package: Package,
    pub bin: Bin,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub entry: String,
}

#[derive(Debug, Deserialize)]
pub struct Bin {
    pub name: String,
}

pub fn load_config(path: &Path) -> Result<Config, String> {
    let config_str = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config: Config = from_str(&config_str).map_err(|e| e.to_string())?;
    Ok(config)
}