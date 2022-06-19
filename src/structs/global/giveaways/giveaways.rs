use chrono::Utc;
use futures::future::join_all;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serenity::{
    builder::CreateEmbed, client::Context, http::CacheHttp, prelude::TypeMapKey, utils::Color,
};
use std::{collections::HashMap, error::Error, fmt::Display, fs, time::Duration};
use tracing::warn;

use crate::structs::{
    config::{styles::StylesConfig, Config},
    functions::{duration_string, get_message},
};

const INDEX_PATH: &str = "./storage/data/global/giveaways/index.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GiveawayData {
    pub message_id: u64,
    pub channel_id: u64,
    pub host: u64,
    pub end: u64,
    pub prize: String,
}

impl GiveawayData {
    pub fn embed<'a>(&self, e: &'a mut CreateEmbed, secs: u64) -> &'a mut CreateEmbed {
        e.title(&self.prize).description(format!(
            "Giveaway ending in {}\nHosted by <@{}>",
            duration_string(&Duration::from_secs(secs),).to_string(),
            self.host
        ))
    }

    pub fn is_active(&self) -> bool {
        (Utc::now().timestamp() as u64) < self.end
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GiveawayIndex(HashMap<u64, Vec<GiveawayData>>);

impl TypeMapKey for GiveawayIndex {
    type Value = GiveawayIndex;
}

impl GiveawayIndex {
    pub fn load() -> Self {
        let index_file = fs::read_to_string(INDEX_PATH).unwrap();
        let index: GiveawayIndex = match serde_json::from_str(&index_file) {
            Ok(index) => index,
            Err(e) => {
                warn!("Failed to parse index file: {}", e);
                GiveawayIndex(HashMap::new())
            }
        };
        index
    }

    pub async fn update_loop(ctx: &Context, interval: Duration, bot_id: u64) {
        loop {
            let updates = {
                let (giveaways, styles) = {
                    let data = ctx.data.read().await;
                    (
                        data.get::<GiveawayIndex>().unwrap().clone(),
                        data.get::<Config>().unwrap().styles,
                    )
                };
                giveaways.edit_all(&ctx, bot_id, styles).await;

                giveaways.check_ended()
            };

            if !updates.is_empty() {
                let mut data = ctx.data.write().await;
                let giveaways = data.get_mut::<GiveawayIndex>().unwrap();
                for (user, index) in updates.into_iter().rev() {
                    giveaways.0.get_mut(&user).unwrap().remove(index);
                }
            }

            tokio::time::sleep(interval).await;
        }
    }

    pub fn add(&mut self, user_id: u64, giveaway: GiveawayData) {
        self.0.entry(user_id).or_insert(Vec::new()).push(giveaway);
    }

    pub fn remove(&mut self, user_id: u64, giveaway_id: u64) {
        if let Some(giveaways) = self.0.get_mut(&user_id) {
            giveaways.retain(|giveaway| giveaway.message_id != giveaway_id);
        }
    }

    pub fn write(&self) -> Result<(), GiveawayError> {
        let index_string = serde_json::to_string_pretty(&self.0)?;
        fs::write(INDEX_PATH, index_string)?;
        Ok(())
    }

    pub async fn edit_all(&self, ctx: &Context, bot_id: u64, styles_config: StylesConfig) {
        let mut futures = Vec::new();
        for giveaways in self.0.values() {
            for giveaway in giveaways.iter() {
                let seconds = if !giveaway.is_active() {
                    0
                } else {
                    giveaway.end - Utc::now().timestamp() as u64
                };

                futures.push(edit_message(ctx, giveaway, seconds, bot_id, styles_config));
            }
        }

        join_all(futures.into_iter()).await;
    }

    pub fn check_ended(&self) -> Vec<(u64, usize)> {
        let mut updates = Vec::new();

        for (user_id, giveaways) in self.0.iter() {
            for (i, giveaway) in giveaways.iter().enumerate() {
                if !giveaway.is_active() {
                    updates.push((*user_id, i));
                }
            }
        }

        updates
    }
}

async fn edit_message(
    ctx: &Context,
    giveaway: &GiveawayData,
    secs: u64,
    bot_id: u64,
    styles_config: StylesConfig,
) -> Result<(), serenity::Error> {
    let mut message = get_message(ctx, giveaway.channel_id, giveaway.message_id).await?;

    if secs == 0 {
        let participants = message
            .reaction_users(&ctx.http, '🎉', None, None)
            .await?
            .into_iter()
            .map(|u| *u.id.as_u64())
            .filter(|id| *id != bot_id)
            .collect::<Vec<_>>();

        // check if anyone has reacted to the message
        if participants.len() == 0 {
            // not enough participants

            message
                .edit(&ctx.http(), |m| {
                    m.embed(|e| {
                        e.title("Giveaway ended")
                            .description("Not enough participants")
                            .color({
                                let c: Color = styles_config.giveaway.into();
                                c
                            })
                    })
                })
                .await?;
        } else {
            let winner = participants[rand::thread_rng().gen_range(0..participants.len())];

            message
                .edit(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("Giveaway ended")
                            .description(format!(
                                "Congratulations <@{}>! You won the giveaway!\nPrize: {}",
                                winner, giveaway.prize
                            ))
                            .color({
                                let c: Color = styles_config.giveaway.into();
                                c
                            })
                            .timestamp(Utc::now())
                    })
                })
                .await?;
        }
    } else {
        message
            .edit(&ctx.http, |m| {
                m.embed(|e| {
                    giveaway.embed(e, secs).color({
                        let c: Color = styles_config.giveaway.into();
                        c
                    })
                })
            })
            .await?;
    }

    Ok(())
}

#[derive(Debug)]
pub enum GiveawayError {
    InvalidTime,
    InvalidCharacterInPrize,
    MissingTime,
    MissingPrize,
    FileError(std::io::Error),
    SerdeError(serde_json::Error),
}

impl Display for GiveawayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GiveawayError::InvalidTime => write!(f, "Invalid time"),
            GiveawayError::InvalidCharacterInPrize => write!(f, "Invalid character in prize"),
            GiveawayError::MissingTime => write!(f, "Missing time"),
            GiveawayError::MissingPrize => write!(f, "Missing prize"),
            GiveawayError::FileError(err) => write!(f, "File error: {}", err),
            GiveawayError::SerdeError(err) => write!(f, "Serde error: {}", err),
        }
    }
}

impl From<std::io::Error> for GiveawayError {
    fn from(err: std::io::Error) -> Self {
        GiveawayError::FileError(err)
    }
}

impl From<serde_json::error::Error> for GiveawayError {
    fn from(err: serde_json::error::Error) -> Self {
        GiveawayError::SerdeError(err)
    }
}

impl Error for GiveawayError {}
