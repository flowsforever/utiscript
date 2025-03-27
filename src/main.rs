use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let config_path = current_dir.join("utconfig.toml");

    if config_path.exists() && config_path.is_file() {
        println!("Found utconfig.toml in the current directory.");
    } else {
        println!("utconfig.toml not found in the current directory.");
    }
}
