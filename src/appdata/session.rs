use std::time::Instant;

use serenity::prelude::TypeMapKey;

pub struct Session {
    pub start: Instant,
}

impl TypeMapKey for Session {
    type Value = Session;
}

impl Session {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}