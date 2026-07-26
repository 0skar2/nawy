use serenity::all::Context;
use serenity::all::Message;
use serenity::builder::EditMessage;
use std::time::Instant;

// ping command to calculate latency :3
pub async fn ping(ctx: &Context, msg: &Message) -> anyhow::Result<()> {
    // start the timer
    let start_time = Instant::now();

    // initial message :3
    let mut response_msg = msg.channel_id.say(&ctx.http, "Pinging.... :3").await?;

    let latency = start_time.elapsed().as_millis();
    let new_content = format!("Pong! Latency is **{}ms** :3", latency);

    response_msg
        .edit(&ctx.http, EditMessage::new().content(new_content))
        .await?;
    Ok(())
}
