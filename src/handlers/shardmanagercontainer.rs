use std::sync::Arc;

use serenity::{client::bridge::gateway::ShardManager, prelude::TypeMapKey};
use tokio::sync::Mutex;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}
