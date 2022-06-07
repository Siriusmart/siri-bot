use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Serialize, Deserialize)]
pub struct MainConfig {
    pub prefix: String,
    pub shard_count: u16,
}

impl Default for MainConfig {
    fn default() -> Self {
        Self {
            prefix: String::from(">"),
            shard_count: 1,
        }
    }
}

impl MainConfig {
    pub fn load() -> Self {
        let config_path = std::env::current_dir()
            .unwrap()
            .join("./storage/config/main.yml");

        let config: MainConfig = if config_path.exists() {
            let config_file = std::fs::read_to_string(&config_path).unwrap();
            match serde_yaml::from_str(&config_file) {
                Ok(config) => config,
                Err(e) => {
                    warn!("Failed to parse config file: {}", e);
                    MainConfig::default()
                }
            }
        } else {
            MainConfig::default()
        };

        std::fs::write(&config_path, serde_yaml::to_string(&config).unwrap()).unwrap();
        config
    }
}
