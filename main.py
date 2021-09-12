import os

import discord
from discord.ext import commands
from pretty_help import PrettyHelp  # Because I'm lazy

from cogs import fun_stuff, server_essentials, utilities, statistics

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


# Cogs go br
client.add_cog(server_essentials.ServerEssentials(client))
client.add_cog(statistics.Statistics(client))
client.add_cog(utilities.Utilities(client))
client.add_cog(fun_stuff.FunStuff(client))

client.run(token)
