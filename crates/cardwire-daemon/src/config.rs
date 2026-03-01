use std::path::Path;
use tokio::fs;
use crate::models::Modes;
use serde::{Deserialize, Serialize};
use toml;
use std::error::Error;

pub const CONFIG_PATH: &str = "/etc/cardwire.toml";

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub mode: Modes,
}

impl Config {
    pub async fn new() -> Config {
        let config_path = Path::new(self::CONFIG_PATH);
        match fs::read_to_string(config_path).await {
            Ok(content) => match toml::from_str(&content) {
                Ok(config) => config,
                Err(_) => Config::default(),
            },
            Err(_) => Config::default(),
        }
    }

    pub fn save_mode_to_config(&self) -> Result<(), Box<dyn Error>> {
        let toml: String = toml::to_string(self).unwrap();
        std::fs::write(CONFIG_PATH, toml)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            // Default to manual mode,
            // it is the most safe option since it doesnt assume the laptop/workstation configuration
            mode: Modes::Manual,
        }
    }
}
