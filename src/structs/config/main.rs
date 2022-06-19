use std::path::Path;

use serde::{Deserialize, Serialize};
use tracing::warn;

// defaults
fn prefix_default() -> String {
    String::from(">")
}

fn shard_count_default() -> u64 {
    1
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MainConfig {
    #[serde(default = "prefix_default")]
    pub prefix: String,

    #[serde(default = "shard_count_default")]
    pub shard_count: u64,
}

impl Default for MainConfig {
    fn default() -> Self {
        Self {
            prefix: prefix_default(),
            shard_count: shard_count_default(),
        }
    }
}

impl MainConfig {
    pub fn load() -> Self {
        let config_path = Path::new("./storage/config/main.yml");

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
