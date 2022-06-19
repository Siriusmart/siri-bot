use siri_bot::{
    appdata::Session,
    commands::{community::COMMUNITY_GROUP, general::GENERAL_GROUP},
    handlers::{after, before, MY_HELP},
    init,
    structs::{config::Config, global::giveaways::giveaways::GiveawayIndex},
};

use serenity::{framework::StandardFramework, http::Http, prelude::GatewayIntents, Client};
use siri_bot::handlers::{Handler, ShardManagerContainer};
use std::{collections::HashSet, env};
use tracing::error;

#[tokio::main]
async fn main() {
    // fs init
    init::init().expect("Could not initialize storage");

    let config = Config::load();

    // load env vars from .env file
    dotenv::dotenv().expect("Failed to load .env file");

    // init logger
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new(&token);

    // owner id
    let (owners, _application_id, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            match http.get_current_user().await {
                Ok(bot_id) => (owners, info.id, bot_id.id),
                Err(e) => panic!("Could not access the bot id: {:?}", e),
            }
        }
        Err(e) => panic!("Could not access application info: {:?}", e),
    };

    // add modules to framework
    let framework = StandardFramework::new()
        .configure(|c| {
            c.prefix(&config.main.prefix)
                .owners(owners)
                .with_whitespace(true)
                .on_mention(Some(bot_id))
        })
        .group(&GENERAL_GROUP)
        .group(&COMMUNITY_GROUP)
        .help(&MY_HELP)
        .before(before)
        .after(after);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .type_map_insert::<Config>(config.clone())
        .type_map_insert::<Session>(Session::new(*bot_id.as_u64()))
        .type_map_insert::<GiveawayIndex>(GiveawayIndex::load())
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(e) = client.start_shards(config.main.shard_count).await {
        error!("Client error: {:?}", e);
    }
}
