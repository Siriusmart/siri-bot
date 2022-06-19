use std::path::Path;

use serde::{Deserialize, Serialize};
use serenity::utils::Color;
use tracing::warn;

// defaults

fn general_default() -> Rgb {
    Rgb::new(82, 114, 255)
}

fn giveaway_default() -> Rgb {
    Rgb::new(255, 130, 245)
}

fn error_default() -> Rgb {
    Rgb::new(145, 0, 2)
}

// Rgb struct

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Into<Color> for Rgb {
    fn into(self) -> Color {
        Color::from_rgb(self.r, self.g, self.b)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct StylesConfig {
    #[serde(default = "general_default")]
    pub general: Rgb,

    #[serde(default = "giveaway_default")]
    pub giveaway: Rgb,

    #[serde(default = "error_default")]
    pub error: Rgb,
}

impl Default for StylesConfig {
    fn default() -> Self {
        Self {
            general: general_default(),
            giveaway: giveaway_default(),
            error: error_default(),
        }
    }
}

impl StylesConfig {
    pub fn load() -> Self {
        let config_path = Path::new("./storage/config/styles.yml");

        let config: StylesConfig = if config_path.exists() {
            let config_file = std::fs::read_to_string(&config_path).unwrap();
            match serde_yaml::from_str(&config_file) {
                Ok(config) => config,
                Err(e) => {
                    warn!("Failed to parse config file: {}", e);
                    StylesConfig::default()
                }
            }
        } else {
            StylesConfig::default()
        };

        std::fs::write(&config_path, serde_yaml::to_string(&config).unwrap()).unwrap();
        config
    }
}
