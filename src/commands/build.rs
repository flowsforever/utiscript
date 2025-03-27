use std::fs;
use std::path::Path;
use crate::config::loader::load_config;
use crate::compiler::parser::parse_ut_file;
use crate::compiler::generator::generate_c_code;

pub fn execute() {
    let config_path = Path::new("utconfig.toml");
    if !config_path.exists() {
        println!("Error: utconfig.toml not found in the current directory.");
        return;
    }

    let config = match load_config(config_path) {
        Ok(config) => config,
        Err(e) => {
            println!("Error loading config: {}", e);
            return;
        }
    };

    let entry_path = Path::new("src").join(&config.package.entry);
    if !entry_path.exists() {
        println!("Error: Entry file {} not found.", entry_path.display());
        return;
    }

    let ast = match parse_ut_file(&entry_path) {
        Ok(ast) => ast,
        Err(e) => {
            println!("Error parsing {}: {}", entry_path.display(), e);
            return;
        }
    };

    let c_code = generate_c_code(&ast);

    let dist_dir = Path::new("dist");
    if !dist_dir.exists() {
        fs::create_dir(dist_dir).expect("Failed to create dist directory");
    }
    let c_file_path = dist_dir.join(format!("{}.c", config.bin.name));
    fs::write(&c_file_path, c_code).expect("Failed to write C file");

    println!("Build successful. C file generated at {}", c_file_path.display());
}