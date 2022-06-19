use std::time::Duration;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{event::ResumedEvent, gateway::Ready},
};
use tracing::info;

use crate::structs::global::giveaways::giveaways::GiveawayIndex;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!(
            "Connected as {}#{}",
            ready.user.name, ready.user.discriminator
        );

        GiveawayIndex::update_loop(&ctx, Duration::from_secs(5), *ready.user.id.as_u64()).await;
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}
