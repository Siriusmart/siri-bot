use std::time::Instant;

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
#[aliases(latency)]
#[description = "Get the bot's latency"]
#[usage = ""]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let start = Instant::now();
    let sent_msg = msg.reply_ping(&ctx.http, "Pong!\nLatency: `Calculating`").await;

    match sent_msg {
        Ok(mut sent_msg) => {
            let end = Instant::now();
            let duration = end.duration_since(start);
            let duration_ms = duration.as_millis();

            sent_msg
                .edit(&ctx.http, |m| {
                    m.content(format!(
                        "Pong!\n\
                Latency: `{}ms`",
                        duration_ms
                    ))
                })
                .await?;
        }
        Err(why) => {
            println!("Error sending message: {:?}", why);
        }
    }
    Ok(())
}
