use serenity::{
    async_trait,
    client::{Client, EventHandler},
    framework::standard::{
        StandardFramework,
        macros::{
            group
        },
    },
};
use std::env;
use kankyo;

mod commands;

use commands::{
    util::*,
    roll::*,
};

#[group]
#[commands(ping, about, roll)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    kankyo::load(false).expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let mut client = Client::new(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
