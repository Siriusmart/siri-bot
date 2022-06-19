use std::time::Duration;

use chrono::Utc;
use duration_string::DurationString;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    utils::Color,
};

use crate::structs::{config::Config, global::giveaways::giveaways::GiveawayIndex};

use crate::structs::global::giveaways::giveaways::{GiveawayData, GiveawayError};

const FAILED_TO_CREATE_GIVEAWAY_MESSAGE: &str = "Failed to create giveaway";

#[command]
#[aliases(start, new)]
#[description = "Start a new giveaway"]
#[usage = "<time> <prize>"]
async fn create(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut data = ctx.data.write().await;
    let giveaways = data.get_mut::<GiveawayIndex>().unwrap();

    let mut sent_msg = msg
        .reply_ping(&ctx.http, format!("Creating giveaway..."))
        .await?;

    let res = || -> Result<GiveawayData, GiveawayError> {
        let giveaway_data = process_args(msg, *sent_msg.id.as_u64(), args)?;
        giveaways.add(giveaway_data.host, giveaway_data.clone());
        giveaways.write()?;
        Ok(giveaway_data)
    }();

    let ok = res.is_ok();

    sent_msg
        .edit(&ctx.http, |m| {
            m.content(String::new()).embed(|mut e| match res {
                Ok(giveaway_data) => {
                    e = e.color({
                        let c: Color = data.get::<Config>().unwrap().styles.giveaway.into();
                        c
                    });
                    giveaway_data.embed(e, giveaway_data.end - Utc::now().timestamp() as u64)
                }
                Err(err) => e
                    .title(FAILED_TO_CREATE_GIVEAWAY_MESSAGE)
                    .description(format!("{}", err))
                    .color({
                        let c: Color = data.get::<Config>().unwrap().styles.error.into();
                        c
                    }),
            })
        })
        .await?;

    if ok {
        sent_msg.react(&ctx.http, '🎉').await?;
    }

    Ok(())
}

fn process_args(
    msg: &Message,
    giveaway_id: u64,
    mut args: Args,
) -> Result<GiveawayData, GiveawayError> {
    let time = args
        .single::<String>()
        .map_err(|_| GiveawayError::MissingTime)?;
    let time: Duration = DurationString::try_from(time)
        .map_err(|_| GiveawayError::InvalidTime)?
        .into();

    let prize = args.rest();
    if prize.is_empty() {
        return Err(GiveawayError::MissingPrize);
    }

    if prize.contains("\n") {
        return Err(GiveawayError::InvalidCharacterInPrize);
    }

    Ok(GiveawayData {
        message_id: giveaway_id,
        host: *msg.author.id.as_u64(),
        end: time.as_secs() + Utc::now().timestamp() as u64,
        prize: prize.to_string(),
        channel_id: *msg.channel_id.as_u64(),
    })
}
