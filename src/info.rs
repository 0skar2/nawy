use serenity::all::Context;
use serenity::all::Message;

pub async fn info(ctx: &Context, msg: &Message) -> anyhow::Result<()> {
    msg.channel_id
        .say(
            &ctx.http,
            "
            **info :3**
            a silly bot made by Energyboy :3
            currently submitted @ stardance by hackclub
            ",
        )
        .await?;
    Ok(())
}
