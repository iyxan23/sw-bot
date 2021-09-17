use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{macros::command, CommandResult, Args};
use serenity::builder::{CreateEmbedAuthor, Timestamp};
use serenity::utils::{MessageBuilder, parse_emoji};
use serenity::model::id::ChannelId;
use tokio::fs::File;
use std::path::Path;
use tokio::io::AsyncWriteExt;
use std::fs::remove_file;

const SHARE_SWB_CHANNEL: u64 = 853779563876319242;

#[command("shareswb")]
#[description = "Share an swb to other people! The swb will be shared on <#853779563876319242>"]
async fn share_swb(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.reply(
            ctx,
            "You need to supply the description as the parameter of the command\n\nExample:\n```+shareswb This is my awesome project!```"
        ).await?;

        return Ok(());
    }

    let description = args.rest();

    if msg.attachments.len() != 1 {
        msg.reply(
            ctx,
            "You need attach one .swb file with your message"
        ).await?;

        return Ok(());
    }

    if !msg.attachments.get(0).unwrap().filename.ends_with(".swb") {
        msg.reply(
            ctx,
            "You need to attach a file with the .swb extension"
        ).await?;

        return Ok(());
    }

    // download the swb file, and then upload it as an attachment
    let swb_file_path = {
        let swb_file = msg.attachments.get(0).unwrap();
        let swb_file_content = swb_file.download().await?;
        let swb_file_download_path = Path::new(&swb_file.filename);

        // now let's write the content to a file
        let mut swb_file = File::create(swb_file_download_path).await?;

        for byte in swb_file_content {
            swb_file.write_u8(byte).await.unwrap();
        }

        swb_file.flush().await.expect("Failed to flush the swb file.");

        swb_file_download_path
    };

    let share_swb_channel = ctx.http.get_channel(SHARE_SWB_CHANNEL).await?;

    share_swb_channel
        .id()
        .send_message(ctx, |m| {
            m.content(
                MessageBuilder::new()
                    .mention(&msg.author)
                    .push(" shared an swb project.")
            ).add_file(swb_file_path)
                .add_embed(|e| {
                    e.author(|a| {
                        a.name(format!("{}#{}", &msg.author.name, &msg.author.discriminator))
                            .icon_url(
                                &msg.author
                                    .avatar_url()
                                    .unwrap_or_else(|| (&msg.author.default_avatar_url()).to_owned())
                            )
                    }).description(description)
                        .footer(|f| {
                            f.text("Send | +shareswb My awesome SWB! | and attach an .swb file along with it to share your own swb!")
                        })
                        .timestamp(&msg.timestamp)
                })
        }).await?;

    msg.delete(ctx).await?;

    // finally remove the downloaded swb file
    remove_file(swb_file_path)?;

    Ok(())
}

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