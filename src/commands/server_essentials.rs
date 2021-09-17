use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{macros::command, CommandResult, Args};
use serenity::builder::{CreateEmbedAuthor, Timestamp};
use serenity::utils::{MessageBuilder, parse_emoji};
use serenity::model::id::ChannelId;

const APP_IDEA_CHANNEL: u64 = 790687893701918730;
const SERVER_IDEA_CHANNEL: u64 = 826514832005136465;
const IDEA_REACTIONS: [&str; 0] = []; // todo: add the upvote and downvote emojis

#[command]
#[description = "Submit an idea to Sketchware Pro; The idea will be sent on <#790687893701918730>."]
#[usage = "(your idea)"]
#[example = "Implement Kotlin to Sketchware Pro"]
async fn idea(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.reply(
            ctx,
            "You need to supply the idea as the parameter of the command\n\nExample:\n```+idea Add this thing into Sketchware Pro```"
        ).await?;

        return Ok(());
    }

    let idea_txt = args.rest().to_string();
    let idea_channel = ctx.http.get_channel(APP_IDEA_CHANNEL).await?;

    send_idea(
        ctx,
        idea_txt,
        idea_channel.id(),
        format!("{}#{}", &msg.author.name, &msg.author.discriminator),
        (&msg.author.avatar_url().unwrap_or_else(|| (&msg.author.default_avatar_url()).to_owned())).to_owned(),
        Timestamp::from(&msg.timestamp)
    ).await?;

    msg.delete(ctx).await?;

    Ok(())
}

#[command("ideaserver")]
#[description = "Submit an idea to the discord server; The idea will be sent on <#826514832005136465>."]
#[usage = "(your idea)"]
#[example = "Add more cat pictures"]
async fn idea_server(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.reply(
            ctx,
            "You need to supply the idea as the parameter of the command\n\nExample:\n```+idea Add more cat pictures```"
        ).await?;

        return Ok(());
    }

    let idea_txt = args.rest().to_string();
    let idea_channel = ctx.http.get_channel(SERVER_IDEA_CHANNEL).await?;

    send_idea(
        ctx,
        idea_txt,
        idea_channel.id(),
        format!("{}#{}", &msg.author.name, &msg.author.discriminator),
        (&msg.author.avatar_url().unwrap_or_else(|| (&msg.author.default_avatar_url()).to_owned())).to_owned(),
        Timestamp::from(&msg.timestamp)
    ).await?;

    msg.delete(ctx).await?;

    Ok(())
}

async fn send_idea(
    ctx: &Context,
    idea_txt: String,
    channel: ChannelId,
    author_display: String,
    author_avatar_url: String,
    timestamp: Timestamp,
) -> CommandResult {
    let idea_msg = channel
        .send_message(ctx, |c| {
            c.add_embed(|e| {
                e.set_author(
                    CreateEmbedAuthor::default()
                        .name(author_display)
                        .icon_url(author_avatar_url)
                        .to_owned()
                ).description(
                    MessageBuilder::new()
                        .push_bold("Idea: ")
                        .push(idea_txt)
                        .push_line(
                            format!("\n\nSend `+idea My awesome idea` to submit your own idea.")
                        )
                ).timestamp(timestamp)
            })
        }).await?;

    for reaction in IDEA_REACTIONS {
        idea_msg.react(
            ctx,
            parse_emoji(reaction).unwrap()
        ).await?;
    }

    Ok(())
}