use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub game: GameConfig,
    pub player: PlayerConfig,
    pub display: DisplayConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    #[serde(default = "default_api_url")]
    pub api_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerConfig {
    #[serde(default = "default_side")]
    pub side: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    #[serde(default = "default_coordinate_format")]
    pub coordinate_format: String,
    #[serde(default = "default_verbosity")]
    pub verbosity: String,
}

fn default_api_url() -> String {
    "http://localhost:3000/api".to_string()
}

fn default_side() -> String {
    "both".to_string()
}

fn default_coordinate_format() -> String {
    "axial".to_string()
}

fn default_verbosity() -> String {
    "normal".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            game: GameConfig {
                api_url: default_api_url(),
            },
            player: PlayerConfig {
                side: default_side(),
            },
            display: DisplayConfig {
                coordinate_format: default_coordinate_format(),
                verbosity: default_verbosity(),
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        // Try to load from config file, fall back to defaults
        match fs::read_to_string("config.toml") {
            Ok(contents) => Ok(toml::from_str(&contents)?),
            Err(_) => {
                eprintln!("No config.toml found, using defaults");
                Ok(Config::default())
            }
        }
    }
}
