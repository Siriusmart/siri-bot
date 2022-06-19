use std::time::Instant;

use serenity::prelude::TypeMapKey;

pub struct Session {
    pub start: Instant,
    pub bot_id: u64,
}

impl TypeMapKey for Session {
    type Value = Session;
}

impl Session {
    pub fn new(bot_id: u64) -> Self {
        Self {
            start: Instant::now(),
            bot_id,
        }
    }
}
