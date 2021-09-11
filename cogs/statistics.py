import discord
from discord.ext import commands


class Statistics(commands.Cog, name="Statistics"):
    """The statistics of the Sketchware Pro discord server"""

    def __init__(self, bot: discord.Client):
        self.bot = bot

    @commands.command(
        name="stats",
        description="The command to show messages statistics of the Sketchware Pro discord server",
        brief="Statistics of the Sketchware Pro discord server",
    )
    async def stats(self, ctx: commands.Context):
        await ctx.channel.send("damb")
