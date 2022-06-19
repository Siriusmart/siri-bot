use futures::future::join;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    utils::Color,
};

use crate::structs::config::Config;

#[command("embed")]
#[description = "Create an embed"]
#[usage = "<content>"]
async fn embed(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let content = args.rest();

    let data = ctx.data.read().await;
    let config = data.get::<Config>().unwrap();

    let send_message = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.description(content).color({
                let c: Color = config.styles.general.into();
                c
            })
        })
    });

    let delete_message = msg.delete(&ctx.http);

    let (output1, output2) = join(delete_message, send_message).await;

    output1?;
    output2?;

    Ok(())
}
