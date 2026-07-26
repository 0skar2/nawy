// import the lobotomy tools
use serenity::all::Context;
use serenity::all::EditMessage;
use serenity::all::Message;
use std::time::Instant;

// functionmaxxing + giving it the message contents
pub async fn pingr(ctx: &Context, msg: &Message) {
    let Some((_, raw_input)) = msg.content.split_once(".pingr ") else {
        return;
    };

    let input = raw_input.trim();
    let target_url = if input.starts_with("http://") || input.starts_with("https://") {
        input.to_string()
    } else {
        format!("https://{}", input)
    };

    let start_time = Instant::now();
    let mut response_msg = match msg.channel_id.say(&ctx.http, "Pinging.... :3").await {
        Ok(m) => m,
        Err(_) => return,
    };

    let response = match reqwest::get(&target_url).await {
        Ok(v) => {
            let latency = start_time.elapsed().as_millis();
            let new_content = format!(" Latency to **{}** is **{}ms** :3", input, latency);
            let builder = EditMessage::new().content(new_content);

            if let Err(why) = response_msg.edit(&ctx.http, builder).await {
                println!("Error editing the message: {why:?}");
            }
            v
        }
        Err(e) => {
            let errormsg = format!("Error: {e:?}");
            let builder = EditMessage::new().content(errormsg);

            if let Err(_why) = response_msg.edit(&ctx.http, builder).await {
                eprintln!("Error: {e:?}");
            }

            return;
        }
    };

    println!("meow shit works ig: {}", response.status());
}
