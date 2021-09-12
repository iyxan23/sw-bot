import copy
import os
import queue

import discord
import psycopg2
from discord.ext import commands, tasks

DATABASE_CREDENTIALS = os.environ['DATABASE_CREDENTIALS']
connection = psycopg2.connect(DATABASE_CREDENTIALS)


class Statistics(commands.Cog, name="Statistics"):
    """The statistics of the Sketchware Pro discord server"""

    def __init__(self, bot: discord.Client):
        self.bot = bot
        self.msg_count_queue = queue.Queue()
        self.publish_changes.start()

    @tasks.loop(minutes=1)
    async def publish_changes(self):
        if self.msg_count_queue.empty():
            return

        # First, we're going to squash these messages into a single map
        squashed = {}

        # todo: somehow clone the queue so we wont like need to process more messages
        #       tried doing deep copy on it but you can't clone the mutex :sad_seizure:
        while not self.msg_count_queue.empty():
            user_id, count = self.msg_count_queue.get()

            if user_id in squashed:
                squashed[user_id] += count
            else:
                squashed[user_id] = count

        # then publish those squashed messages count into the database
        with connection.cursor() as cur:
            for user_id, count in squashed.items():
                # check if the user is in the messages count table
                cur.execute("SELECT EXISTS (SELECT 1 FROM messages_count WHERE user_id = %s)", (user_id, ))

                if cur.fetchone()[0]:  # fetchone() returns (False,)
                    # ok then increment the value
                    cur.execute("UPDATE messages_count SET count = count + %s WHERE user_id = %s", (count, user_id))
                else:
                    # then create the row
                    cur.execute("INSERT INTO messages_count (user_id, count) VALUES (%s, %s)", (user_id, count))

            # commit the changes
            connection.commit()

    @commands.Cog.listener()
    async def on_message(self, message: discord.Message):
        if message.author.bot:
            # ignore messages by bots
            return

        # todo: create a table that stores ignored channels
        if message.channel.id == 814216752196091985:
            return

        self.msg_count_queue.put_nowait((message.author.id, 1))

    @commands.command(
        name="stats",
        description="The command to show messages statistics of the Sketchware Pro discord server",
        brief="Statistics of the Sketchware Pro discord server",
    )
    async def stats(self, ctx: commands.Context):
        result = ""

        with connection.cursor() as cur:
            cur.execute("SELECT * FROM messages_count ORDER BY count DESC LIMIT 5")

            while (entry := cur.fetchone()) is not None:
                print(entry)
                result += f"{entry[0]} - {entry[1]} total messages\n"

        await ctx.send(result)
