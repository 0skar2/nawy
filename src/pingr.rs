// import the lobotomy tools
use anyhow::anyhow;
use reqwest::Url;
use serenity::all::Context;
use serenity::all::EditMessage;
use serenity::all::Message;
use std::time::Instant;

// functionmaxxing + giving it the message contents
pub async fn pingr(ctx: &Context, msg: &Message, args: &[&str]) -> anyhow::Result<()> {
    if args.is_empty() {
        return Err(anyhow!("At least 1 argument is required"));
    }

    let mut url = Url::parse(args[0])?;

    if !["http", "https"].contains(&url.scheme()) {
        url.set_scheme("https").unwrap();
    }

    let start_time = Instant::now();
    let mut response_msg = msg.channel_id.say(&ctx.http, "Pinging.... :3").await?;

    match reqwest::get(url.clone()).await {
        Ok(_response) => {
            let latency = start_time.elapsed().as_millis();
            let new_content = format!(" Latency to **{}** is **{}ms** :3", url, latency);
            response_msg
                .edit(&ctx.http, EditMessage::new().content(new_content))
                .await?;
        }
        Err(err) => {
            response_msg
                .edit(
                    &ctx.http,
                    EditMessage::new().content(format!("Error: {err:?}")),
                )
                .await?;
        }
    };
    Ok(())
}
