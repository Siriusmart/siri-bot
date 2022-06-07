use serenity::framework::standard::macros::group;

mod ping;
use ping::PING_COMMAND;

mod say;
use say::SAY_COMMAND;

mod prefix;
use prefix::PREFIX_COMMAND;

mod uptime;
use uptime::UPTIME_COMMAND;

#[group]
#[description = "General commands"]
#[commands(ping, say, prefix, uptime)]
pub struct General;