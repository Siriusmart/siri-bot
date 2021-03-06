use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    utils::Color,
};

use crate::{
    appdata::Session,
    structs::{config::Config, functions::duration_string},
};

#[command]
#[description = "Get the bot's session uptime"]
#[usage = ""]
async fn uptime(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let session = data.get::<Session>().unwrap();
    let config = data.get::<Config>().unwrap();

    let content = format!("Uptime: `{}`", duration_string(&session.start.elapsed()));

    let mut sent_msg = msg.reply_ping(&ctx.http, &content).await?;

    sent_msg
        .edit(&ctx.http, |m| {
            m.content(String::new()).embed(|e| {
                e.title("Bot Uptime").description(&content).color({
                    let c: Color = config.styles.general.into();
                    c
                })
            })
        })
        .await?;
    Ok(())
}
