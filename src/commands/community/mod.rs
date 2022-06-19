// mod poll;
mod giveaway;
use giveaway::GIVEAWAY_COMMAND;
use serenity::framework::standard::macros::group;

#[group]
#[description = "Community commands"]
#[commands(giveaway)]
pub struct Community;
