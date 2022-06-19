use std::time::Instant;

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    utils::Color,
};
use tracing::info;

use crate::structs::config::Config;

#[command]
#[aliases(latency)]
#[description = "Get the bot's latency"]
#[usage = ""]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let start = Instant::now();
    let sent_msg = msg.reply_ping(&ctx.http, "Calculating...").await;

    match sent_msg {
        Ok(mut sent_msg) => {
            let data = ctx.data.read().await;
            let config = data.get::<Config>().unwrap();

            let end = Instant::now();
            let duration = end.duration_since(start);
            let duration_ms = duration.as_millis();

            sent_msg
                .edit(&ctx.http, |m| {
                    m.content(String::new()).embed(|e| {
                        e.title("Pong!")
                            .description(format!("Latency: `{}ms`", duration_ms))
                            .color({
                                let c: Color = config.styles.general.into();
                                c
                            })
                    })
                })
                .await?;
        }
        Err(why) => {
            info!("Error sending message: {:?}", why);
        }
    }
    Ok(())
}
