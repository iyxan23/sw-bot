mod commands;
mod database;

use serenity::{
    async_trait,
    client::{
        Client,
        Context,
        EventHandler
    },
    model::{
        channel::{
            Message
        },
        event::{
            ResumedEvent
        },
        gateway::{
            Ready
        },
        id::{
            UserId
        }
    },
    framework::standard::{
        StandardFramework,
        CommandResult,
        macros::{
            group,
            help,
            hook
        },
        Args,
        HelpOptions,
        CommandGroup,
        help_commands
    },
};

use std::env;
use std::collections::HashSet;

use commands::{
    utilities::*,
    fun_stuff::*,
    server_essentials::*,
    statistics::*,
};
use serenity::model::prelude::{Activity, OnlineStatus};

#[group("Server Essentials")]
#[commands(idea, idea_server, share_swb)]
#[description = "Server-specific commands like idea, ideaserver etc"]
struct ServerEssentials;

#[group]
#[commands(stats)]
#[description = "Statistics of the Sketchware Pro discord server"]
struct Statistics;

#[group]
#[commands(howgay, howgeh, interject, uninterject, carbon)]
#[description = "Fun commands to play around with"]
struct FunStuff;

#[group]
#[commands(ping, whoami, purge, spurge)]
#[description = "Utility commands"]
struct Utilities;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, bot: Ready) {
        ctx.shard.set_activity(
            Option::from(
                Activity::streaming(
                    "Get real",
                    "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
                // pretend i am not here, this is an absolutely normal video and nothing is wrong
                )
            )
        );

        ctx.shard.set_status(OnlineStatus::Idle);

        println!("{}#{} is ready", bot.user.name, bot.user.discriminator);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("+"))
        .after(after)
        .group(&SERVERESSENTIALS_GROUP)
        .group(&STATISTICS_GROUP)
        .group(&FUNSTUFF_GROUP)
        .group(&UTILITIES_GROUP)
        .help(&HELP);

    let token = env::var("DISCORD_BOT_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    if let Err(why) = command_result {
        println!("Command '{}' returned error {:?}: {}", command_name, why, why.to_string());

        let result = _msg.channel_id.say(_ctx,
            format!(
                "The command `{}` failed to run and returned an error, this error will be forwarded to the devs.",
                command_name
            )
        ).await;

        if result.is_err() {
            println!("Failed to send the error message, {:?}", result.unwrap_err());
        }
    }
}

#[help]
#[individual_command_tip = "\
Hello, If you want more information about a specific command, just pass the command as argument.\n
Something like
```
+help ping
```"]
#[command_not_found_text = "Could not find: `{}`."]
#[embed_success_colour = "#349afe"]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(
        context,
        msg,
        args,
        help_options,
        groups,
        owners
    ).await;

    Ok(())
}