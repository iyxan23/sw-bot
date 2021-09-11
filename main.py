import os

import discord
from discord.ext import commands
from pretty_help import PrettyHelp  # Because I'm lazy

from cogs import fun_stuff, server_essentials, utilities

client = commands.Bot(
    command_prefix="+",
    description="Sketchware Pro's official discord bot",
    help_command=PrettyHelp(color=discord.colour.Colour(0x349afe))
)

token = os.getenv("DISCORD_BOT_TOKEN")


@client.event
async def on_ready():
    await client.change_presence(status=discord.Status.idle, activity=discord.Game("Sketchware Pro"))
    print("SWBot is ready")


@client.event
async def on_message(message):
    if await process_message(message):
        await client.process_commands(message)


async def process_message(message):
    if message.author.id == 155149108183695360:
        if message.content.endswith("Watch your language."):
            await message.channel.send("dyno, stop it, get some help", delete_after=5)
            return False
    else:
        if message.author.bot: return True
        if len(message.content) < 8: return True

        if message.content[:8].lower() == "petition":
            await message.add_reaction('<:upvote:833702317098008646>')
            await message.add_reaction('<:downvote:833702170306150440>')
            return False
        else:
            return True


@client.event
async def on_reaction_add(reaction, user):
    emojis = ['<:upvote:833702317098008646>', '<:downvote:833702170306150440>']

    # Check if this is us 
    if reaction.message.author == client.user and user != client.user:
        if reaction.emoji in emojis:
            for react_ in reaction.message.reactions:
                if react_.emoji == reaction.emoji:
                    continue

                users = await react_.users().flatten()
                if user in users:
                    await reaction.message.remove_reaction(react_.emoji, user)
                    return


# Cogs go br
client.add_cog(utilities.Utilities(client))
client.add_cog(fun_stuff.FunStuff(client))
client.add_cog(server_essentials.ServerEssentials(client))

client.run(token)
