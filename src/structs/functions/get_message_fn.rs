use serenity::{client::Context, model::channel::Message};

pub async fn get_message(
    ctx: &Context,
    channel_id: u64,
    message_id: u64,
) -> Result<Message, serenity::Error> {
    Ok(match ctx.cache.as_ref().message(channel_id, message_id) {
        Some(message) => message,
        None => ctx.http.get_message(channel_id, message_id).await?,
    })
}
