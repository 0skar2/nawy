use serenity::all::Context;
use serenity::all::Message;
use std::time::SystemTime;

pub async fn time(ctx: &Context, msg: &Message) -> anyhow::Result<()> {
    let timenow = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    msg.channel_id
        .say(&ctx.http, format!("The current time is <t:{}>", timenow))
        .await?;
    Ok(())
}
