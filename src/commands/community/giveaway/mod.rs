mod create;

use crate::structs::config::Config;
use create::CREATE_COMMAND;
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    utils::Color,
};

#[command]
#[aliases(gway, giveaways)]
#[sub_commands(create)]
#[description = "Pick a winner from the list of people who have entered the giveaway after a given time"]
#[usage = ""]
async fn giveaway(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let config = data.get::<Config>().unwrap();

    let content = String::from("This is the giveaway command");
    let mut sent_msg = msg.reply_ping(&ctx.http, &content).await?;

    sent_msg
        .edit(&ctx.http, |m| {
            m.content(String::new()).embed(|e| {
                e.title("Giveaways").description(&content).color({
                    let c: Color = config.styles.giveaway.into();
                    c
                })
            })
        })
        .await?;

    Ok(())
}
