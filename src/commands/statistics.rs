use serenity::framework::standard::{CommandResult, macros::command};
use serenity::model::channel::Message;
use serenity::client::Context;
use serenity::utils::Colour;

#[command]
#[description = "Shows the statistics of the Sketchware Pro discord server"]
#[only_in("guilds")]
#[usage = ""]
#[example = ""]
#[aliases("statistics")]
async fn stats(ctx: &Context, msg: &Message) -> CommandResult {
    // todo: implement the database

    msg.channel_id
        .send_message(ctx, |c| {
            c.add_embed(|e| {
                e.title("Statistics")
                    .description("Statistics of the Sketchware Pro discord server")
                    .field("Active Members", "WIP", true)
                    .field("Messages", "WIP", true)
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