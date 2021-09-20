use serenity::framework::standard::{CommandResult, macros::command};
use serenity::model::channel::Message;
use serenity::client::Context;
use serenity::utils::Colour;
use crate::database::SWBotDatabase;

#[command]
#[description = "Shows the statistics of the Sketchware Pro discord server"]
#[only_in("guilds")]
#[usage = ""]
#[example = ""]
#[aliases("statistics")]
async fn stats(ctx: &Context, msg: &Message) -> CommandResult {
    let read = ctx.data.read().await;
    let db = read.get::<SWBotDatabase>().unwrap();

    let (active_members, active_members_messages): (String, String) = async {
        let active_members_vec = db.list_active_members().await;
        let mut active_members = String::new();
        let mut active_members_messages = String::new();

        for member in active_members_vec {
            active_members
                .push_str(
                    format!("<@{}>\n", member.user_id.0).as_str()
                );

            active_members_messages
                .push_str(
                    format!("{}\n", member.count).as_str()
                )
        }

        (active_members, active_members_messages)
    }.await;

    msg.channel_id
        .send_message(ctx, |c| {
            c.add_embed(|e| {
                e.title("Statistics")
                    .description("Statistics of the Sketchware Pro discord server")
                    .field("Active Members", active_members, true)
                    .field("Messages", active_members_messages, true)
                    .field("Total Messages", "WIP", false)
                    .field("Average Messages", "WIP", true)
                    .field("Today", "WIP", true)
                    .footer(|f| {
                        f.text(
                            format!(
                                "Requested by {}#{}",
                                &msg.author.name, &msg.author.discriminator
                            )
                        )
                    })
                    .colour(Colour(0x3499fe))
            })
        }).await?;

    Ok(())
}