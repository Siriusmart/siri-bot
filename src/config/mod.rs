mod main;
use main::MainConfig;
use serenity::prelude::TypeMapKey;

pub struct Config {
    pub main: MainConfig,
}

impl TypeMapKey for Config {
    type Value = Config;
}

impl Config {
    pub fn load() -> Self {
        Self {
            main: MainConfig::load(),
        }
    }
}
