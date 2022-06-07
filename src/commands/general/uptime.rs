use human_duration::human_duration;
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::appdata::Session;

#[command]
#[description = "Get the bot's session uptime"]
#[usage = ""]
async fn uptime(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let session = data.get::<Session>().unwrap();

    msg.reply_ping(
        &ctx.http,
        &format!(
            "I've been up for {}",
            human_duration(&session.start.elapsed())
        ),
    )
    .await?;

    Ok(())
}
