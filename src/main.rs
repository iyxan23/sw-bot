use serenity::async_trait;
use serenity::client::{ Client, Context, EventHandler };
use serenity::model::{
    channel::{
        Message
    },
    event::{
        ResumedEvent
    },
    gateway::{
        Ready
    }
};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use std::env;
use std::time::Instant;

#[group]
#[commands(ping)]
struct General;

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
        .group(&GENERAL_GROUP);

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

#[command]
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