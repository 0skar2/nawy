// nawy
// my first attempt at a discord bot written in rust, lord save us all

// imports :3
use anyhow::anyhow;
use serenity::async_trait; // Required for Serenity v0.12+
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;
struct Handler;
mod ping;
use ping::ping;
mod coinflip;
use coinflip::coinflip;
mod hackclub;
use hackclub::hackclub;
mod help;
use help::help;
mod randcat;
use randcat::cat;
mod time;
use time::time;
mod info;
use info::info;
mod pingr;
use pingr::pingr;

#[async_trait]
// implementation of event handler so when message contents == something -> then something happens
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id != 1527332908287656036 {
            let message = msg.content.clone();
            if message.starts_with(".") {
                let list: Vec<&str> = message.split(" ").collect();
                let cmd = list[0];
                let args = &list[1..];
                if let Err(err) = match cmd {
                    "help" => help(&ctx, &msg).await,
                    "coinflip" => coinflip(&ctx, &msg).await,
                    "hackclub" => hackclub(&ctx, &msg).await,
                    "cat" => cat(&ctx, &msg).await,
                    "time" => time(&ctx, &msg).await,
                    "info" => info(&ctx, &msg).await,
                    "ping" => ping(&ctx, &msg).await,
                    "pingr" => pingr(&ctx, &msg, args).await,
                    _ => Err(anyhow!("I dont know this command")),
                } {
                    let _ = msg.channel_id.say(&ctx.http, format!("{}", err)).await;
                }
            } else {
                // automeower :3
                // meow list :p
                let meows = vec!["meow", "mrow", "nya", "mrrrp", "prr", "purr"];

                // the thing that checks if message is meowing :3
                if meows.iter().any(|mrow| msg.content.contains(mrow)) {
                    let _ = msg.channel_id.say(&ctx.http, "meow:3c").await;
                };
            }
        }
    }

    // async fn ready(&self, _: Context, ready: Ready) {
    //     println!("{} is connected :3", ready.user.name)
    // }
}

#[tokio::main]
async fn main() {
    // config client with the discord bot token in the .env
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("expected a token in the .env");

    // set gateway intents or big discord won't be happy
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // create new instance of client logging in as the bot
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating a client");

    // finafuckingly start a single shard and start listening to events :3
    if let Err(why) = client.start().await {
        println!("client error: {why:?}");
    }
}
