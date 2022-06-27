use std::fs;
use std::process::exit;

use serde_derive::Deserialize;
use serde_derive::Serialize;

const CONFIGFILE: &str = ".club-elo.toml";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub user_names: Vec<String>,
}

pub fn read_config() -> Config {
    let contents = match fs::read_to_string(CONFIGFILE) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not read file {}: {}", CONFIGFILE, e);
            exit(1);
        }
    };
    let config = match toml::from_str(&contents) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not parse config file: {}", e);
            exit(1);
        }
    };

    return config;
}
