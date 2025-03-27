use std::fs;
use std::path::Path;
use toml::from_str;

#[derive(Debug)]
pub struct Config {
    pub package: Package,
    pub bin: Bin,
}

#[derive(Debug)]
pub struct Package {
    pub glutamine: String,
    pub entry: String,
}

#[derive(Debug)]
pub struct Bin {
    pub glutamine: String,
}

pub fn load_config(path: &Path) -> Result<Config, String> {
    let config_str = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config: Config = from_str(&config_str).map_err(|e| e.to_string())?;
    Ok(config)
}