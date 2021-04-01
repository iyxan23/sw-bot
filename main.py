import discord
from discord.ext import commands
import os
import datetime

client = commands.Bot(command_prefix="+")
token = os.getenv("DISCORD_BOT_TOKEN")


@client.event
async def on_ready():
    await client.change_presence(status = discord.Status.idle, activity = discord.Game("me por modir"))
    print("Discord bot ready")



@client.command(name="ping", description="What do you think this would be?")
async def ping(ctx):
    await ctx.send(f"🏓 Pong, that took {int(client.latency * 1000)}ms")

@client.command(name="pong", description="This is like ping but with a surprise")
async def pong(ctx):
    await ctx.send(f"🏓 Ping, that took {int(client.latency * 1000)}ms"[::-1])

@client.command(name="whoami", description="Who am I? Who are you!? WHERE AM I?!? WHY AM I HERE?!?1?!1?!")
async def whoami(ctx):
    await ctx.send(f"You're {ctx.message.author.name}, dum dum")



@client.command(name="idea", description="Suggest an idea, Usage: +idea app/server \"Hello World\"")
async def idea(ctx, idea_for="app", idea=None):
    channel = None

    if idea_for == "server":
        channel = client.get_channel(826514832005136465)

    elif idea_for == "app":
        channel = client.get_channel(790687893701918730)

    else:
        await ctx.send("Hey, the 1st parameter can only be \"app\" (suggest something for the mod) or \"server\" (suggest something for the server).")
        return

    if idea == None:
        await ctx.send("Hey, can you put your idea on the 2rd argument?")
        return

    emojis = ['⬆️', '⬇️']

    embed = discord.Embed(description="**Idea:** " + idea, color=0x1891fb)
    embed.set_author(name=ctx.message.author.display_name, icon_url=ctx.message.author.avatar_url)
    embed.timestamp = datetime.datetime.utcnow()
    embed.set_footer(text="SWBot")

    message = await ctx.send(embed=embed)

    await ctx.message.delete()
    
    for emoji in emojis:
        await message.add_reaction(emoji)



@client.event
async def on_reaction_add(reaction, user):
    emojis = ['⬆️', '⬇️']

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

client.run(token)
