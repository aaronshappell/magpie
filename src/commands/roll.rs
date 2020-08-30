use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        CommandResult,
        macros::command,
    },
};
use regex::Regex;
use rand::{
    Rng,
    distributions::{Distribution ,Uniform},
};

#[command]
async fn roll(ctx: &Context, msg: &Message) -> CommandResult {
    // Check for rick roll 1%
    if rand::thread_rng().gen::<f32>() < 0.01 {
        msg.reply(ctx, "You tried to roll a 'die' but got 'rick'\nhttps://www.youtube.com/watch?v=dQw4w9WgXcQ").await?;
        return Ok(())
    }

    // Load die parameters
    let mut params: Vec<&str> = msg.content.trim().split(" ").collect();
    params.remove(0);

    // Check for emtpy input
    if params.len() == 0 {
        msg.reply(ctx, "Please input a die to roll").await?;
        return Ok(())
    }

    // Generate rolls for each parameter
    let mut rolls: Vec<String> = Vec::new();
    let re = Regex::new(r"(?P<amount>\d*)d(?P<sides>\d+)\+?(?P<modifier>\d*)").unwrap();
    for param in &params {
        // Regex parsing
        let cap = match re.captures(param) {
            Some(c) => c,
            None => {
                msg.reply(ctx, format!("'{}' is not a valid die", param)).await?;
                return Ok(())
            },
        };

        // Further parsing for amount, sides, and modifier
        let amount: u32 = match &cap["amount"] {
            "0" => {
                msg.reply(ctx, format!("'{}' is not a valid die", param)).await?;
                return Ok(())
            }
            "" => 1,
            _ => cap["amount"].parse().unwrap()
        };
        let sides: u32 = match &cap["sides"] {
            "0" => {
                msg.reply(ctx, format!("'{}' is not a valid die", param)).await?;
                return Ok(())
            }
            _ => cap["sides"].parse().unwrap()
        };
        let modifier: u32 = match &cap["modifier"] {
            "" => 0,
            _ => cap["modifier"].parse().unwrap()
        };

        // Sample # of rolls from distribution
        let mut rng = rand::thread_rng();
        let dist = Uniform::new_inclusive(1, sides);
        let samples: Vec<u32> = dist.sample_iter(&mut rng).take(amount as usize).collect();

        // Format roll printout
        let mut roll = String::from("(");
        let mut i = 0;
        while i < samples.len() - 1 {
            roll.push_str(&format!("{}, ", samples[i]));
            i += 1;
        }
        roll.push_str(&format!("{})", samples[i]));
        if modifier != 0 {
            roll.push_str(&format!(" + {}", modifier));
        }
        roll.push_str(&format!(" = {}", samples.into_iter().sum::<u32>() + modifier));
        rolls.push(roll);
    }

    // Compile rolls into one bot reply
    let mut reply = String::from("\n");
    for i in 0..rolls.len() {
        reply.push_str(&format!("You rolled '{}' and got '{}'\n", params[i], rolls[i]));
    }
    msg.reply(ctx, &reply).await?;

    Ok(())
}