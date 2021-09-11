import datetime
import time

import discord
from discord.ext import commands


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


class ServerEssentials(commands.Cog, name="Server Essentials"):
    """Sketchware Pro discord server specific commands like idea, shareswb, etc"""

    def __init__(self, bot):
        self.bot = bot
        self.idea_cool_downs = {}

    @commands.command(
        name="idea",
        description="Suggest an idea for Sketchware Pro",
        brief="Suggest an idea for ẞketchware Pro"
    )
    async def idea(self, ctx, *argv):
        try:
            if self.idea_cool_downs[ctx.author.id]:
                if self.idea_cool_downs[ctx.author.id] > time.time():
                    await ctx.message.reply(
                        f"you need to wait {string_time_thing(int(self.idea_cool_downs[ctx.author.id] - time.time()))} before submitting another idea <:sadTroll:849956944478208010>",
                        delete_after=30)
                    return
                else:
                    del self.idea_cool_downs[ctx.author.id]
        except KeyError as e:
            pass

        self.idea_cool_downs[ctx.author.id] = time.time() + 30 * 60  # 30 mins cooldown

        channel = self.bot.get_channel(790687893701918730)

        if len(argv) == 0:
            await ctx.send("You need to put in your idea on the first argument")
            return

        idea = " ".join(argv)

        emojis = ['<:upvote:833702317098008646>', '<:downvote:833702170306150440>']

        embed = discord.Embed(
            description=f"**Idea:** {idea}\n\nSend `+idea your idea` in <#814828261044650064> to do this",
            color=0x1891fb)
        embed.set_author(name=f"{ctx.message.author.display_name}#{ctx.message.author.discriminator}",
                         icon_url=ctx.message.author.avatar_url)
        embed.timestamp = datetime.datetime.utcnow()

        message = await channel.send(embed=embed)

        await ctx.message.delete()

        for emoji in emojis:
            await message.add_reaction(emoji)

    @commands.command(
        name="ideaserver",
        description="Suggest an idea for the server",
        brief="Suggest an idea for the server"
    )
    async def ideaserver(self, ctx, *argv):
        try:
            if self.idea_cool_downs[ctx.author.id]:
                if self.idea_cool_downs[ctx.author.id] > time.time():
                    await ctx.message.reply(
                        f"you need to wait {string_time_thing(int(self.idea_cool_downs[ctx.author.id] - time.time()))} before submitting another idea <:sadTroll:849956944478208010>",
                        delete_after=30)
                    return
                else:
                    del self.idea_cool_downs[ctx.author.id]
        except KeyError as e:
            pass

        self.idea_cool_downs[ctx.author.id] = time.time() + 30 * 60  # 30 mins cooldown

        channel = self.bot.get_channel(826514832005136465)

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

    @commands.command(
        name="shareswb",
        description="Share your swb project to #creations-swb",
        brief="Share your swb project to #creations-swb"
    )
    async def shareswb(self, ctx, *args):
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
        channel = self.bot.get_channel(853779563876319242)

        embed = discord.Embed(description=text, color=0x0da3e3)
        embed.set_author(name=ctx.message.author.display_name, icon_url=ctx.message.author.avatar_url)
        await channel.send(content=f"{ctx.message.author.mention} shared an swb project", embed=embed, file=file_)

        await ctx.message.delete()
        await wait.delete()

    @commands.command(
        name="stats",
        description="This command shows the messages statistics in this server",
        brief="Shows messages statistics"
    )
    async def stats(self, ctx):
        await ctx.message.channel.send("damb")
