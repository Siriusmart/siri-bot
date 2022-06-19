use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    utils::Color,
};

use crate::structs::config::Config;

#[command]
#[description = "Get the bot's prefix"]
#[usage = ""]
async fn prefix(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let config = data.get::<Config>().unwrap();

    let content = &format!("My prefix is `{}`", config.main.prefix);

    let mut msg = msg.reply_ping(&ctx.http, &content).await?;

    msg.edit(&ctx.http, |m| {
        m.content(String::new()).embed(|e| {
            e.title("Bot Prefix").description(&content).color({
                let c: Color = config.styles.general.into();
                c
            })
        })
    })
    .await?;
    Ok(())
}
