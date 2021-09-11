from discord.ext import commands


class Utilities(commands.Cog, name="Utilities"):
    def __init__(self, bot):
        self.bot = bot

    @commands.command(
        name="ping",
        description="What do you think this would be?",
        brief="Check the bot's ping"
    )
    async def ping(self, ctx):
        await ctx.send(f"🏓 Pong, that took {int(self.bot.latency * 1000)}ms")

    @commands.command(
        name="pong",
        description="This is like ping but with a surprise",
        brief="Like ping but with a surprise"
    )
    async def pong(self, ctx):
        await ctx.send(f"🏓 Ping, that took {int(self.bot.latency * 1000)}ms"[::-1])

    @commands.command(
        name="whoami",
        description="Who am I? Who are you!? WHERE AM I?!? WHY AM I HERE?!?1?!1?!",
        brief="Who are you?"
    )
    async def whoami(self, ctx):
        await ctx.send(f"You're {ctx.message.author.name}, dum dum")

    @commands.command(
        name="purge",
        description="Delete messages, I guess",
        brief="Delete messages"
    )
    async def purge(self, ctx, amount=1):
        if ctx.message.author.guild_permissions.manage_messages:
            if amount < 0:
                await ctx.send(content="you")
                return
            await ctx.message.delete()
            await ctx.channel.purge(limit=amount)
            await ctx.send(content="Purged " + str(amount) + " messages", delete_after=10)
        else:
            await ctx.send("Where is your \"Manage Messages\" permission <:wtfwithtea:826512739949084754>")

    @commands.command(
        name="spurge",
        alias="sp",
        description="Like purge but silent",
        brief="Delete messages silently"
    )
    async def spurge(self, ctx, amount=1):
        if ctx.message.author.guild_permissions.manage_messages:
            if amount < 0:
                await ctx.message.author.send(content=f"why {amount}?????!!?!?!?!????!??!??!?!?!?!??!?!?!??!?")
                return
            await ctx.message.delete()
            await ctx.channel.purge(limit=amount)
        else:
            await ctx.message.author.send(
                "Where is your \"Manage Messages\" permission nub <:wtfwithtea:826512739949084754>")

    async def get_webhook(self, ctx):
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
