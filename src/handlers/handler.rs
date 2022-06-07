use serenity::{client::{EventHandler, Context}, model::{gateway::Ready, event::ResumedEvent}, async_trait};
use tracing::info;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}#{}", ready.user.name, ready.user.discriminator);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}