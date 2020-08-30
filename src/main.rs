use serenity::{
    async_trait,
    prelude::*,
    model::prelude::*,
    client::{Client, EventHandler},
    framework::standard::{
        StandardFramework,
        CommandResult,
        macros::{
            group,
            command,
        },
    },
};
use std::env;
use kankyo;

mod commands;
mod whois;

use commands::{
    util::*,
    roll::*,
};

#[group]
#[commands(ping, about, roll, whois, checkem)]
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

#[command]
async fn whois(ctx: &Context, msg: &Message) -> CommandResult {
    let name = &(msg.content)[7..];
    let r = whois::get_message(name);
    match r {
        Ok(m) => { msg.reply(ctx, m).await?; },
        Err(e) => { msg.reply(ctx, e).await?; },
    }
    Ok(())
}

#[command]
async fn checkem(ctx: &Context, msg: &Message) -> CommandResult {
    println!("Rolling: {:?}", msg.id);
    let num : &u64 = msg.id.as_u64();

    let reply;
    if (num % 100) % 11 == 0 {
        reply = format!("You rolled a `{}`. Checked n' kek'd, my friend.", num);
    } else {
        reply = format!("You rolled a `{}`. No digits. Sad.", num);
    }

    msg.reply(ctx, reply).await?;

    Ok(())
}
