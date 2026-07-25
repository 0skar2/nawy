// import the lobotomy tools
use reqwest::Error;
use serenity::all::Context;
use serenity::all::EditMessage;
use serenity::all::Message;
use std::time::Instant;

// functionmaxxing + giving it the message contents
pub async fn pingr(ctx: &Context, msg: &Message) {
    let url = msg.content.split_once(" ");
    let start_time = Instant::now();
    let response_msg = msg.channel_id.say(&ctx.http, "Pinging.... :3").await;
    let response = match reqwest::get("https://gayboi.club").await {
        Ok(v) => {
            let latency = start_time.elapsed().as_millis();
            let new_content = format!(" Latency to **gayboi.club** is **{}ms** :3", latency);
            let builder = EditMessage::new().content(new_content);

            if let Err(why) = response_msg.expect("REASON").edit(&ctx.http, builder).await {
                println!("Error editing the message: {why:?}");
            }
            v
        }
        Err(e) => {
            let errormsg = format!("Error: {e:?}");
            let builder = EditMessage::new().content(errormsg);

            if let Ok(mut sent_msg) = response_msg {
                if let Err(why) = sent_msg.edit(&ctx.http, builder).await {
                    eprintln!("Error: {e:?}");
                }
            }

            return;
        }
    };

    println!("meow shit works ig: {}", response.status());
}
