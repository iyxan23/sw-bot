use serenity::framework::standard::{macros::command, CommandResult, Args};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Result;
use std::time::Instant;
use serenity::futures::StreamExt;
use tokio::time::Duration;

#[command]
#[description = "\
Delete messages in bulk.
This can only be used for members that has the \"Manage Messages\" permission.

Delete 10 messages (excluding the purge command message):
```
+purge 10
```
Related command(s): `spurge`
"]
#[required_permissions(MANAGE_MESSAGES)]
async fn purge(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let count_res = args.single::<i32>();

    if count_res.is_err() {
        msg.reply(
            ctx,
            format!("Failed parsing the 1st argument as an integer: {}", count_res.unwrap_err())
        ).await?;
    } else {
        let count = count_res.unwrap();

        if count > 1000 {
            msg.reply(ctx, "Woah, that's too big my dude").await?;

            return Ok(())
        }

        let now = Instant::now();

        delete_messages(ctx, &msg.channel_id, count + 1).await?;
                                                 /* +1 for the command message */

        let message = msg.channel_id.send_message(ctx, |c| {
            c.content(format!("{} message(s) deleted in {}ms", count, now.elapsed().as_millis()))
        }).await?;

        // delay for 5 secs and delete the message
        tokio::time::sleep(Duration::from_secs(5)).await;

        message.delete(ctx).await?;
    }

    Ok(())
}

#[command]
#[description = "\
Delete messages in bulk _silently_.
This can only be used for members that has the \"Manage Messages\" permission.

Delete 10 messages (excluding the purge command message):
```
+spurge 10
```
Related command(s): `purge`
"]
#[required_permissions(MANAGE_MESSAGES)]
async fn spurge(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let count_res = args.single::<i32>();

    if count_res.is_err() {
        msg.reply(
            ctx,
            format!("Failed parsing the 1st argument as an integer: {}", count_res.unwrap_err())
        ).await?;
    } else {
        let count = count_res.unwrap();

        if count > 1000 {
            msg.reply(ctx, "Woah, that's too big my dude").await?;

            return Ok(())
        }

        // and just delete them
        delete_messages(ctx, &msg.channel_id, count + 1).await?;
                                                     /* +1 for the command message */
    }

    Ok(())
}

async fn delete_messages(ctx: &Context, channel: &ChannelId, count: i32) -> Result<()> {
    let mut message_ids: Vec<MessageId> = Vec::new();

    // this might not be the most efficient way to do it lmao
    {
        let mut messages = channel.messages_iter(&ctx).boxed();

        for _ in 0..count {
            let message = messages.next().await;
            message_ids.push(message.unwrap()?.id);
        }
    }

    channel.delete_messages(ctx, message_ids).await?;

    Ok(())
}

#[command]
#[description = "Shows the ping of the bot"]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let start = Instant::now();
    let mut msg = msg.reply(ctx, ":ping_pong: Pong").await?;
    let duration = start.elapsed();

    msg.edit(
        ctx,
        |edit_msg|
            edit_msg.content(
                format!(":ping_pong: Pong, that took {}ms", (duration.as_millis()))
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