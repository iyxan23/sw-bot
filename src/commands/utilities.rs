use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Instant;

#[command]
#[description = "Shows the ping of the bot"]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let start = Instant::now();
    let mut msg = msg.reply(ctx, "Pong! :ping_pong:").await?;
    let duration = start.elapsed();

    msg.edit(
        ctx,
        |edit_msg|
            edit_msg.content(
                format!("Pong! :ping_pong: That took {}ms", (duration.as_millis()))
            )
    ).await?;

    Ok(())
}

#[command]
#[description = "When you forgot who you are"]
async fn whoami(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, format!("You're {}, dum dum", msg.author.name)).await?;

    Ok(())
}