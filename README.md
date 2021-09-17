# Sketchware Pro's Discord Bot
Sketchware Pro's discord bot re-written in rust. The old python code can be located on the branch `python-legacy`.

## Running
We do not nor planning on distributing any binaries for this bot, so you will need to build it yourself.

 1. [Install rust](https://www.rust-lang.org/tools/install)
 2. Run
    ```console
    $ cargo build --release
    ```
 3. Navigate to the directory `target/release`
 4. And run the bot with the `DISCORD_BOT_TOKEN` variable (make sure to change the variable to be your own bot's token)
    ```console
    $ DISCORD_BOT_TOKEN="discord bot token" ./sw-bot
    ```
 5. Have fun with the bot

## Why rewrite to rust?
If you haven't seen yet, [discord.py is not maintained anymore](https://gist.github.com/Rapptz/4a2f62751b9600a31a0d3c78100287f1) and we will need to switch to another library that's still maintained. discord.js is a good option but I'm currently in the interest of learning rust.. so, I chose the [serenity](https://github.com/serenity-rs/serenity) rust library to re-write this bot.
