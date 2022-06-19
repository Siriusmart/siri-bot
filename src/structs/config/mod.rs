pub mod main;
pub mod styles;

use main::MainConfig;
use serenity::prelude::TypeMapKey;
use styles::StylesConfig;

#[derive(Debug, Clone)]
pub struct Config {
    pub main: MainConfig,
    pub styles: StylesConfig,
}

impl TypeMapKey for Config {
    type Value = Config;
}

impl Config {
    pub fn load() -> Self {
        Self {
            main: MainConfig::load(),
            styles: StylesConfig::load(),
        }
    }
}
