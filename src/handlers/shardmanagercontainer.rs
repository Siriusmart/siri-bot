use std::sync::Arc;

use serenity::{prelude::TypeMapKey, client::bridge::gateway::ShardManager};
use tokio::sync::Mutex;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}