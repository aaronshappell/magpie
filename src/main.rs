use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

mod tokens;

mod whois;


#[commands(ping,whois,checkem)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let mut client = Client::new(tokens::DISCORD_TOKEN)
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
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
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
