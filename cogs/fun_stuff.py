import random
import time

import discord
from discord.ext import commands

with open("../interjection.txt", "r") as f:
    interjection = f.read()

with open("../uninterjection.txt", "r") as f:
    rs = f.read().split("|||||")

    uninterjection = rs[0]
    uninterjection2 = rs[1]


class FunStuff(commands.Cog, name="Fun stuff"):
    def __init__(self, bot):
        self.bot = bot
        self.interject_cool_downs = {}
        self.utils = self.bot.get_cog("Utilities")

    @commands.command(
        name="howgay",
        description="How gay are you <:lmao:792845009400758272>",
        brief="self-explanatory"
    )
    async def howgay(self, ctx, who=None):
        if who is None:
            who = ctx.author.mention

        gay = random.randint(0, 101)

        if gay == 69:
            description = f"{who} IS {gay}% GAY!?.!?. w.- T. F.?.!1?!?1..!? :flushed: :flushed: " \
                          f"<:uhm:815635169616068609> <:uhm:815635169616068609> "

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

    @commands.command(
        name="howgeh",
        description="How geh are you <:lmao:792845009400758272>",
        brief="    g    e    h    "
    )
    async def howgeh(self, ctx, who=None):
        if who is None:
            who = ctx.author.mention

        geh = random.randint(0, 100)

        if geh == 69:
            description = f"{who} IS {geh}% GEHHH!?.!?. w.- T. F.?.!1?!?1..!?"
        else:
            description = f"{who} is {geh}%   g  e  h  "

        embed = discord.Embed(title="  g  e  h  detector machine", description=description, color=0xeb0fc6)
        await ctx.send(embed=embed)

    @commands.command(
        name="interject",
        description="i would like to interject you for a moment",
        brief="did someone said linux?"
    )
    async def interject(self, ctx):
        try:
            if self.interject_cool_downs[ctx.author.id]:
                if self.interject_cool_downs[ctx.author.id] > time.time():
                    await ctx.channel.send(f"Whoa, slow down {ctx.author.mention}!", delete_after=5)
                    return
                else:
                    del self.interject_cool_downs[ctx.author.id]
        except KeyError:
            pass

        self.interject_cool_downs[ctx.author.id] = time.time() + 600  # 10 mins cooldown

        await ctx.message.delete()

        webhook = await self.utils.get_webhook(ctx.channel)

        await webhook.send(interjection, username=ctx.message.author.display_name,
                           avatar_url=ctx.message.author.avatar_url)

    @commands.command(
        name="uninterject",
        description="no richard, it's linux",
        brief="did someone has just interjected?"
    )
    async def uninterject(self, ctx):
        try:
            if self.interject_cool_downs[ctx.author.id]:
                if self.interject_cool_downs[ctx.author.id] > time.time():
                    await ctx.channel.send(f"Whoa, slow down {ctx.author.mention}!", delete_after=5)
                    return
                else:
                    del self.interject_cool_downs[ctx.author.id]

        except KeyError:
            pass

        self.interject_cool_downs[ctx.author.id] = time.time() + 600  # 10 mins cooldown

        await ctx.message.delete()

        webhook = await self.utils.get_webhook(ctx.channel)

        await webhook.send(
            uninterjection,
            username=ctx.message.author.display_name,
            avatar_url=ctx.message.author.avatar_url
        )

        await webhook.send(
            uninterjection2,
            username=ctx.message.author.display_name,
            avatar_url=ctx.message.author.avatar_url
        )
