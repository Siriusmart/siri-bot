use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::config::Config;

#[command]
#[description = "Get the bot's prefix"]
#[usage = ""]
async fn prefix(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let config = data.get::<Config>().unwrap();

    msg.reply_ping(&ctx.http, &format!("My prefix is `{}`", config.main.prefix))
        .await?;

    Ok(())
}
