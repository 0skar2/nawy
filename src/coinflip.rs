use serenity::all::Context;
use serenity::all::Message;

//coinflip command :3
pub async fn coinflip(ctx: &Context, msg: &Message) -> anyhow::Result<()> {
    let result = if rand::random() {
        "Heads :3"
    } else {
        "Tails :3"
    };
    msg.channel_id.say(&ctx.http, result).await?;
    Ok(())
}
