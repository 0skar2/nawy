use serenity::all::Context;
use serenity::all::Message;

pub async fn help(ctx: &Context, msg: &Message) -> anyhow::Result<()> {
    msg.channel_id
        .say(
            &ctx.http,
            "
            List of commands :3c
            - .help -> displays this menu
            - .coinflip -> does a coinflip :3
            - .cat -> returns random picture of a cat :3
            - .time -> tells the current time :3
            - .info -> displays some info
            - .ping -> pings server
            - .pingr -> pings remote server (currently only http & https),
            - (meow at bot and it will meow back :3)
            ",
        )
        .await?;
    Ok(())
}
