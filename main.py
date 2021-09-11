import os

import discord
from discord.ext import commands
from pretty_help import PrettyHelp  # Because I'm lazy
import re
import random
import datetime
import time
import messages_count

client = commands.Bot(command_prefix="+", help_command=PrettyHelp())
token = os.getenv("DISCORD_BOT_TOKEN")

with open("interjection.txt", "r") as f:
    interjection = f.read()

with open("uninterjection.txt", "r") as f:
    rs = f.read().split("|||||")

    uninterjection = rs[0]
    uninterjection2 = rs[1]


def string_time_thing(time):
    # copied this from stackoverflow :troll:
    seconds_in = {
        'year': 365 * 24 * 60 * 60,
        'month': 30 * 24 * 60 * 60,
        'day': 24 * 60 * 60,
        'hour': 60 * 60,
        'minute': 60
    }

    years, rem = divmod(time, seconds_in['year'])
    months, rem = divmod(rem, seconds_in['month'])
    days, rem = divmod(rem, seconds_in['day'])
    hours, rem = divmod(rem, seconds_in['hour'])
    minutes, rem = divmod(rem, seconds_in['minute'])
    seconds = rem

    return (f"{years} year(s) " if years != 0 else "") + \
           (f"{months} month(s) " if months != 0 else "") + \
           (f"{days} day(s) " if days != 0 else "") + \
           (f"{hours} hour(s) " if hours != 0 else "") + \
           (f"{minutes} minute(s) " if minutes != 0 else "") + \
           (f"{seconds} second(s)" if seconds != 0 else "")


async def get_webhook(ctx):
    webhooks = await ctx.webhooks()

    webhook = False

    for webhook_ in webhooks:
        if webhook_.name == "SWPro-pro-webhook":
            webhook = webhook_
            break

    if not webhook:
        webhook = await ctx.create_webhook(
            name="SWPro-pro-webhook",
            reason="amogus sus used to impersonate peop- i mean, to make jokes"
        )

    return webhook


@client.event
async def on_ready():
    await client.change_presence(status=discord.Status.idle, activity=discord.Game("me por modir"))
    print("Discord bot ready")


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


idea_cooldowns = {}


@client.command(
    name="idea",
    description="Suggest an idea for Sketchware Pro",
    brief="Suggest an idea for ẞketchware Pro"
)
async def idea(ctx, *argv):
    try:
        if idea_cooldowns[ctx.author.id]:
            if idea_cooldowns[ctx.author.id] > time.time():
                await ctx.message.reply(
                    f"you need to wait {string_time_thing(int(idea_cooldowns[ctx.author.id] - time.time()))} before submitting another idea <:sadTroll:849956944478208010>",
                    delete_after=30)
                return
            else:
                del idea_cooldowns[ctx.author.id]
    except KeyError as e:
        pass

    idea_cooldowns[ctx.author.id] = time.time() + 30 * 60  # 30 mins cooldown

    channel = client.get_channel(790687893701918730)

    if len(argv) == 0:
        await ctx.send("You need to put in your idea on the first argument")
        return

    idea = " ".join(argv)

    emojis = ['<:upvote:833702317098008646>', '<:downvote:833702170306150440>']

    embed = discord.Embed(description=f"**Idea:** {idea}\n\nSend `+idea your idea` in <#814828261044650064> to do this",
                          color=0x1891fb)
    embed.set_author(name=f"{ctx.message.author.display_name}#{ctx.message.author.discriminator}",
                     icon_url=ctx.message.author.avatar_url)
    embed.timestamp = datetime.datetime.utcnow()

    message = await channel.send(embed=embed)

    await ctx.message.delete()

    for emoji in emojis:
        await message.add_reaction(emoji)


@client.command(
    name="ideaserver",
    description="Suggest an idea for the server",
    brief="Suggest an idea for the server"
)
async def ideaserver(ctx, *argv):
    try:
        if idea_cooldowns[ctx.author.id]:
            if idea_cooldowns[ctx.author.id] > time.time():
                await ctx.message.reply(
                    f"you need to wait {string_time_thing(int(idea_cooldowns[ctx.author.id] - time.time()))} before submitting another idea <:sadTroll:849956944478208010>",
                    delete_after=30)
                return
            else:
                del idea_cooldowns[ctx.author.id]
    except KeyError as e:
        pass

    idea_cooldowns[ctx.author.id] = time.time() + 30 * 60  # 30 mins cooldown

    channel = client.get_channel(826514832005136465)

    if len(argv) == 0:
        await ctx.send("You need to put in your idea on the first argument")
        return

    idea = " ".join(argv)

    emojis = ['<:upvote:833702317098008646>', '<:downvote:833702170306150440>']

    embed = discord.Embed(
        description=f"**Idea:** {idea}\n\nSend `+ideaserver your idea` in <#814828261044650064> to do this",
        color=0x1891fb)
    embed.set_author(name=f"{ctx.message.author.display_name}#{ctx.message.author.discriminator}",
                     icon_url=ctx.message.author.avatar_url)
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
async def spurge(ctx, amount=1):
    if ctx.message.author.guild_permissions.manage_messages:
        if amount < 0:
            await ctx.message.author.send(content=f"why {amount}?????!!?!?!?!????!??!??!?!?!?!??!?!?!??!?")
            return
        await ctx.message.delete()
        await ctx.channel.purge(limit=amount)
    else:
        await ctx.message.author.send(
            "Where is your \"Manage Messages\" permission nub <:wtfwithtea:826512739949084754>")


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

    elif 0 < gay < 25:
        description = f"{who} is {gay}% gay <:manhehe:828189506880536626>"

    elif 24 < gay < 50:
        description = f"{who} is {gay}% gay.. <:bruhmonkey:828189406703779861>"

    elif 49 < gay < 75:
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

    if geh == 69:
        description = f"{who} IS {geh}% GEHHH!?.!?. w.- T. F.?.!1?!?1..!?"
    else:
        description = f"{who} is {geh}%   g  e  h  "

    embed = discord.Embed(title="  g  e  h  detector machine", description=description, color=0xeb0fc6)
    await ctx.send(embed=embed)


@client.command(
    name="interject",
    description="i would like to interject you for a moment",
    brief="did someone said linux?"
)
async def interject(ctx):
    try:
        if interject_cooldowns[ctx.author.id]:
            if interject_cooldowns[ctx.author.id] > time.time():
                await ctx.channel.send(f"Whoa, slow down {ctx.author.mention}!", delete_after=5)
                return
            else:
                del interject_cooldowns[ctx.author.id]
    except KeyError as e:
        pass

    interject_cooldowns[ctx.author.id] = time.time() + 600  # 10 mins cooldown

    await ctx.message.delete()

    webhook = await get_webhook(ctx.channel)

    await webhook.send(interjection, username=ctx.message.author.display_name, avatar_url=ctx.message.author.avatar_url)


interject_cooldowns = {}


@client.command(
    name="uninterject",
    description="no richard, it's linux",
    brief="did someone has just interjected?"
)
async def uninterject(ctx):
    try:
        if interject_cooldowns[ctx.author.id]:
            if interject_cooldowns[ctx.author.id] > time.time():
                await ctx.channel.send(f"Whoa, slow down {ctx.author.mention}!", delete_after=5)
                return
            else:
                del interject_cooldowns[ctx.author.id]
    except KeyError as e:
        pass

    interject_cooldowns[ctx.author.id] = time.time() + 600  # 10 mins cooldown

    await ctx.message.delete()

    webhook = await get_webhook(ctx.channel)

    await webhook.send(uninterjection, username=ctx.message.author.display_name,
                       avatar_url=ctx.message.author.avatar_url)
    await webhook.send(uninterjection2, username=ctx.message.author.display_name,
                       avatar_url=ctx.message.author.avatar_url)


@client.command(
    name="shareswb",
    description="Share your swb project to #creations-swb",
    brief="Share your swb project to #creations-swb"
)
async def shareswb(ctx, *args):
    text = " ".join(args)

    if len(ctx.message.attachments) == 0:
        await ctx.message.reply("You need to attach an swb file")
        return

    if len(ctx.message.attachments) > 1:
        await ctx.message.reply("You can't attach multiple files")
        return

    if not ctx.message.attachments[0].filename.endswith(".swb"):
        split = ctx.message.attachments[0].filename.split(".")
        await ctx.message.reply(f"You need to attach an .swb file, not .{split[len(split) - 1]}")
        return

    wait = await ctx.message.reply(f"alright, give me a few secs to download the file")

    file_ = await ctx.message.attachments[0].to_file()
    channel = client.get_channel(853779563876319242)

    embed = discord.Embed(description=text, color=0x0da3e3)
    embed.set_author(name=ctx.message.author.display_name, icon_url=ctx.message.author.avatar_url)
    await channel.send(content=f"{ctx.message.author.mention} shared an swb project", embed=embed, file=file_)

    await ctx.message.delete()
    await wait.delete()


@client.command(
    name="stats",
    description="This command shows the messages statistics in this server",
    brief="Shows messages statistics"
)
async def stats(ctx):
    await ctx.message.channel.send("damb")


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
