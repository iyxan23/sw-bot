mod commands;

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
            help
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
    fun_stuff::*
};

#[group]
#[commands(ping, whoami)]
#[description = "Utility commands"]
struct Utilities;

#[group]
#[commands(interject, uninterject)]
#[description = "Fun commands to play around with"]
struct FunStuff;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, bot: Ready) {
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
        .group(&UTILITIES_GROUP)
        .group(&FUNSTUFF_GROUP)
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