use futures::future::join;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    utils::{content_safe, ContentSafeOptions},
};

#[command]
#[description = "Says something"]
#[usage = "<text>"]
async fn say(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let settings = if let Some(guild_id) = msg.guild_id {
        ContentSafeOptions::default()
            .clean_channel(false)
            .display_as_member_from(guild_id)
    } else {
        ContentSafeOptions::default()
            .clean_channel(false)
            .clean_role(false)
    };

    let content = content_safe(&ctx.cache, &args.rest(), &settings, &msg.mentions);

    let delete_message = msg.delete(&ctx.http);

    let send_message = msg.channel_id.say(&ctx.http, &content);

    let (output1, output2) = join(delete_message, send_message).await;

    output1?;
    output2?;

    Ok(())
}
