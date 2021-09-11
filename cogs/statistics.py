import os

import discord
import psycopg2
from discord.ext import commands

DATABASE_CREDENTIALS = os.environ['DATABASE_CREDENTIALS']
connection = psycopg2.connect(DATABASE_CREDENTIALS)


class Statistics(commands.Cog, name="Statistics"):
    """The statistics of the Sketchware Pro discord server"""

    def __init__(self, bot: discord.Client):
        self.bot = bot

    @commands.Cog.listener()
    async def on_message(self, message: discord.Message):
        if message.author.bot:
            return

        return  # todo: count messages in batch and update the values once every some, maybe 10 minutes to reduce load

        user_id = message.author.id

        cur = connection.cursor()

        # check if the user is in the messages count table
        if cur.execute("SELECT EXISTS (SELECT 1 FROM messages_count WHERE user_id = %s)", (user_id, )):
            # ok then increment the value
            cur.execute("UPDATE messages_count SET count = count + 1 WHERE user_id = %s", (user_id, ))
        else:
            # then create the table
            cur.execute("INSERT INTO messages_count (user_id, count) VALUES (%s, %s)", (user_id, 1))

        connection.commit()
        cur.close()

    @commands.command(
        name="stats",
        description="The command to show messages statistics of the Sketchware Pro discord server",
        brief="Statistics of the Sketchware Pro discord server",
    )
    async def stats(self, ctx: commands.Context):
        await ctx.channel.send("damb")
