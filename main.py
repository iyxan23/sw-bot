import discord
from discord.ext import commands
from pretty_help import PrettyHelp # Because I'm lazy
import os
import random
import datetime

client = commands.Bot(command_prefix="+", help_command=PrettyHelp())
token = os.getenv("DISCORD_BOT_TOKEN")


@client.event
async def on_ready():
    await client.change_presence(status = discord.Status.idle, activity = discord.Game("me por modir"))
    print("Discord bot ready")

@client.event
async def on_message(message):
    if (await process_message(message)): await client.process_commands(message)


async def process_message(message):
    if message.author.id == 155149108183695360:
        if message.content.endswith("Watch your language."):
            await message.channel.send("dyno, stop it, get some help", delete_after=5)
            return False
    else:
        if message.author.bot: return False
        if len(message.content) < 8: return True
        if message.content[:8].lower() == "petition":
            await message.add_reaction('<:upvote:833702317098008646>')
            await message.add_reaction('<:downvote:833702170306150440>')
            return False

@client.command(
        name="ping",
        description="What do you think this would be?",
        brief="Check the bot's ping"
)
async def ping(ctx):
    await ctx.send(f"🏓 Pong, that took {int(client.latency * 1000)}ms")

@client.command(
        name="pong",
        description="This is like ping but with a surprise",
        brief="Like ping but with a surprise"
)
async def pong(ctx):
    await ctx.send(f"🏓 Ping, that took {int(client.latency * 1000)}ms"[::-1])

@client.command(
        name="whoami",
        description="Who am I? Who are you!? WHERE AM I?!? WHY AM I HERE?!?1?!1?!",
        brief="Who are you?"
)
async def whoami(ctx):
    await ctx.send(f"You're {ctx.message.author.name}, dum dum")



@client.command(
        name="idea",
        description="Suggest an idea for the app or the server",
        brief="Suggest an idea"
)
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

    emojis = ['<:upvote:833702317098008646>', '<:downvote:833702170306150440>']

    embed = discord.Embed(description=f"**Idea:** {idea}\n\nSend `+idea " + idea_for + " \"your idea\"` in <#814828261044650064> to do this", color=0x1891fb)
    embed.set_author(name=ctx.message.author.display_name, icon_url=ctx.message.author.avatar_url)
    embed.timestamp = datetime.datetime.utcnow()

    message = await channel.send(embed=embed)

    await ctx.message.delete()
    
    for emoji in emojis:
        await message.add_reaction(emoji)

@client.command(
        name="purge",
        description="Delete messages, I guess",
        brief="Delete messages"
)
async def purge(ctx, amount=1):
    if ctx.message.author.guild_permissions.manage_messages:
        if amount < 0:
            await ctx.send(content="you")
            return
        await ctx.message.delete()
        await ctx.channel.purge(limit=amount)
        await ctx.send(content="Purged " + str(amount) + " messages", delete_after=10)
    else:
        await ctx.send("Where is your \"Manage Messages\" permission <:wtfwithtea:826512739949084754>")

@client.command(
        name="spurge",
        alias="sp",
        description="Like purge but silent",
        brief="Delete messages silently"
)
async def purge(ctx, amount=1):
    if ctx.message.author.guild_permissions.manage_messages:
        if amount < 0:
            await ctx.message.author.send(content=f"why {amount}?????!!?!?!?!????!??!??!?!?!?!??!?!?!??!?")
            return
        await ctx.message.delete()
        await ctx.channel.purge(limit=amount)
    else:
        await ctx.message.author.send("Where is your \"Manage Messages\" permission nub <:wtfwithtea:826512739949084754>")


@client.command(
        name="howgay",
        description="How gay are you <:lmao:792845009400758272>",
        brief="self-explanatory"
)
async def howgay(ctx, who=None):
    if who is None:
        who = ctx.author.mention

    description = ""

    gay = random.randint(0, 101)

    if gay == 69:
        description = f"{who} IS {gay}% GAY!?.!?. w.- T. F.?.!1?!?1..!? :flushed: :flushed: <:uhm:815635169616068609> <:uhm:815635169616068609>"

    elif gay > 0 and gay < 25:
        description = f"{who} is {gay}% gay <:manhehe:828189506880536626>"

    elif gay > 24 and gay < 50:
        description = f"{who} is {gay}% gay.. <:bruhmonkey:828189406703779861>"

    elif gay > 49 and gay < 75:
        description = f"{who} is {gay}% gay! :flushed:"

    elif gay == 101:
        description = f"OMG RUNNNN {who} IS {gay}% GAY !! AAAAA <:jerryshock:798202239784583198>"

    else:
        description = f"{who} is {gay}% gay!!! :flushed: <:uhm:815635169616068609>"

    embed = discord.Embed(title="Gay detector machine", description=description, color=0xeb0fc6)
    await ctx.send(embed=embed)

@client.command(
        name="howgeh",
        description="How geh are you <:lmao:792845009400758272>",
        brief="    g    e    h    "
)
async def howgeh(ctx, who=None):
    if who is None:
        who = ctx.author.mention

    description = ""

    geh = random.randint(0, 100)

    if gay == 69:
        description = f"{who} IS {geh}% GEHHH!?.!?. w.- T. F.?.!1?!?1..!?"
    else:
        description = f"{who} is {geh}%   g  e  h  "

    embed = discord.Embed(title="  g  e  h  detector machine", description=description, color=0xeb0fc6)
    await ctx.send(embed=embed)


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

client.run(token)
