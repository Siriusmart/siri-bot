use serenity::{
    client::Context,
    framework::standard::{macros::hook, CommandResult},
    model::channel::Message,
};
use tracing::{error, info};

#[hook]
pub async fn after(_: &Context, msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => info!(
            "Processed command `{}` by `{}#{}`",
            command_name, msg.author.name, msg.author.discriminator
        ),
        Err(e) => error!(
            "Command `{}` by `{}#{}` returned error {:?}",
            command_name, msg.author.name, msg.author.discriminator, e
        ),
    }
}
