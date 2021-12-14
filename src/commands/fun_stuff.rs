use serenity::framework::standard::{CommandResult, macros::command, Args};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Error;
use serenity::utils::{MessageBuilder, ArgumentConvert};
use serenity::utils::Colour;

#[command]
#[description = "Shows how gay you / the person you mentioned are"]
#[only_in("guilds")]
#[usage = "[optional: user id / @mention / UserName#Tag]"]
#[example = ""]
#[example = "@lahds13"]
#[aliases("gay")]
#[min_args(0)]
#[max_args(1)]
async fn howgay(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let user = if args.is_empty() {
        msg.author.clone()
    } else {
        Member::convert(
            ctx,
            msg.guild_id,
            Option::Some(msg.channel_id),
            &args.current().unwrap()
        ).await?.user
    };

    // make the percentage based on the user's id, since.. each user must have their own
    // preferences.. right?

    // here, we will get the user id and modulo it with 101
    let number = user.id.0 % 101u64;
    let mut description = MessageBuilder::new();
    description.mention(&user);

    if number == 0 { description.push(" is straight, dam"); }
    else if number < 25 { description.push(format!(" is {}% gay", number)); }
    else if number < 50 { description.push(format!(" is {}% gay! mama mia!", number)); }
    else if number < 75 { description.push(format!(" is {}% gay! 😳", number)); }
    else if number < 90 { description.push(format!(" IS {}% GAY! <:uhm:815635169616068609>", number)); }
    else {
        description.push_bold(
            format!(
                " IS {} GAY! <:hels3_1:870207027031453706><:hels3_2:870207286415618059><:hels3_3:870207322188840990>",
                number
            )
        );
    }

    msg.channel_id.send_message(ctx, |c| {
        c.reference_message(msg)
            .add_embed(|e| {
                e.title("real no scam gay detector machine")
                    .description(description)
                    .colour(Colour::new(0xEB0FC6))
            })
    }).await?;

    Ok(())
}

#[command]
#[description = "Shows how      g     e     h      a person are"]
#[only_in("guilds")]
#[usage = "[optional: user id / @mention / UserName#Tag]"]
#[example = ""]
#[example = "@lahds13"]
#[aliases("geh")]
#[min_args(0)]
#[max_args(1)]
async fn howgeh(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let user = if args.is_empty() {
        msg.author.clone()
    } else {
        Member::convert(
            ctx,
            msg.guild_id,
            Option::Some(msg.channel_id),
            args.current().unwrap()
        ).await?.user
    };

    // make the percentage based on the user's id, since.. each user must have their own
    // preferences.. right?

    // here will be a bit different, we're going to take the (7th + 8th) and (10th + 11th) number
    // of the user id, multiply it by each other and modulo it with 101
    let number = {
        let user_id = user.id.0.to_string();

        let first_number: i32 = user_id.get(7..8).unwrap().parse().unwrap();
        let second_number: i32 = user_id.get(10..11).unwrap().parse().unwrap();

        (first_number * second_number) % 101
    };

    let mut description = MessageBuilder::new();
    description.mention(&user);

    if number == 0 { description.push(" is absolutely not geh. dam, true man"); }
    else if number < 25 { description.push(format!(" is {}% geh", number)); }
    else if number < 50 { description.push(format!(" is {}% g  e  h", number)); }
    else if number == 69 { description.push(format!(" i-sss {}% gEH!?.!?. w.- T. F.?.!1?!?1..!?", number)); }
    else if number < 75 { description.push(format!(" is {}% G E H 😳", number)); }
    else if number < 90 { description.push(format!(" IS {}% GEEH <:uhm:815635169616068609>", number)); }
    else {
        description.push_bold(
            format!(
                " IS {} GEEEEHHH! <:uhm:815635169616068609><:uhm:815635169616068609>",
                number
            )
        );
    }

    msg.channel_id.send_message(ctx, |c| {
        c.reference_message(msg)
            .add_embed(|e| {
                e.title("g  e  h  detector machine")
                    .description(description)
                    .colour(Colour::new(0xEB0FC6))
            })
    }).await?;

    Ok(())
}
#[command]
#[description = "Beautify a snippet"]
#[usage = "(your snippet)"]
#[example = "println('Hello World')"]

async fn carbon(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.reply(
            ctx,
            "You need to supply a code as the parameter of the command\n\nExample:\n```+carbon println('apple')```"
        ).await?;

        return Ok(());
    }
    let code = args.rest().to_string().replace(" ", "+").replace("\n","+%0A+");
    let url : String = "https://carbonnowsh.herokuapp.com/?code=".to_owned() + &code + "&theme=darcula&backgroundColor=rgba(255,255,255)";

        msg.channel_id.send_message(&ctx, |m | {
        m.embed(|e | {
            e.image(url);
            return e;
        });

        return m;
    }
    ).await?;

    Ok(())
}

#[command]
#[description = "Did someone has just said linux?"]
#[only_in("guilds")]
#[usage = ""]
#[example = ""]
#[aliases("i")]
#[num_args(0)]
async fn interject(ctx: &Context, msg: &Message) -> CommandResult {
    // retrieve the webhook
    let webhook: Webhook = get_webhook(ctx, msg.channel_id).await?;
    let msg_c: Message = msg.clone();

    let avatar_url: Option<String> = msg_c.author.avatar_url();
    let name: String = msg_c.author.name;

    // send the interjection text
    webhook.execute(ctx, false, |x| {
        x   .content(INTERJECTION)
            .username(&name);

        if !avatar_url.is_none() { x.avatar_url(avatar_url.as_ref().unwrap()); }

        x
    }).await?;

    // then remove the original command
    msg.delete(ctx).await?;

    Ok(())
}

#[command]
#[description = "Did someone has just interjected?"]
#[only_in("guilds")]
#[usage = ""]
#[example = ""]
#[aliases("u")]
#[num_args(0)]
async fn uninterject(ctx: &Context, msg: &Message) -> CommandResult {
    // retrieve the webhook
    let webhook: Webhook = get_webhook(ctx, msg.channel_id).await?;
    let msg_c = msg.clone();

    let avatar_url: &Option<String> = &msg_c.author.avatar_url();
    let name: &String = &msg_c.author.name;

    // send the first part of the un-interjection (since it's too long, we needed to split it to two parts)
    webhook.execute(ctx, false, |x| {
        x   .content(UN_INTERJECTION_1)
            .username(name);

        if !avatar_url.is_none() { x.avatar_url(avatar_url.as_ref().unwrap()); }

        x
    }).await?;

    // and the second one
    webhook.execute(ctx, false, |x| {
        x   .content(UN_INTERJECTION_2)
            .username(name);

        if !avatar_url.is_none() { x.avatar_url(avatar_url.as_ref().unwrap()); }

        x
    }).await?;

    // then remove the original command
    msg.delete(ctx).await?;

    Ok(())
}

const WEBHOOK_NAME: &str = "SWBot-pro-webhook";

/// This function retrieves the Bot's webhook from the specified channel, if there's none,
/// it'll create one
async fn get_webhook(ctx: &Context, channel: ChannelId) -> std::result::Result<Webhook, Error> {
    let webhooks: Vec<Webhook> = channel.webhooks(ctx).await?;

    let webhook = webhooks.iter().find(
        |v|
            v.name.as_ref().unwrap()
                ==
            &WEBHOOK_NAME.to_string()
    );

    if webhook.is_none() {
        Ok(channel.create_webhook(ctx, WEBHOOK_NAME).await?)
    } else {
        Ok(webhook.unwrap().clone())
    }
}

/* Interjection and un-interjection texts */
const INTERJECTION: &str = "\
I'd just like to interject for a moment. What you're referring to as Linux, is in fact, GNU/Linux, \
or as I've recently taken to calling it, GNU plus Linux. Linux is not an operating system unto \
itself, but rather another free component of a fully functioning GNU system made useful by the GNU \
corelibs, shell utilities and vital system components comprising a full OS as defined by POSIX.

Many computer users run a modified version of the GNU system every day, without realizing it. \
Through a peculiar turn of events, the version of GNU which is widely used today is often called \
Linux, and many of its users are not aware that it is basically the GNU system, developed by the \
GNU Project.

There really is a Linux, and these people are using it, but it is just a part of the system they \
use. Linux is the kernel: the program in the system that allocates the machine's resources to the \
other programs that you run. The kernel is an essential part of an operating system, but useless \
by itself; it can only function in the context of a complete operating system. Linux is normally \
used in combination with the GNU operating system: the whole system is basically GNU with Linux \
added, or GNU/Linux. All the so-called Linux distributions are really distributions of GNU/Linux!
";

const UN_INTERJECTION_1: &str = "No, Richard, it's 'Linux', not 'GNU/Linux'. The most important \
contributions that the FSF made to Linux were the creation of the GPL and the GCC compiler. Those \
are fine and inspired products. GCC is a monumental achievement and has earned you, RMS, and the \
Free Software Foundation countless kudos and much appreciation.

Following are some reasons for you to mull over, including some already answered in your FAQ.

One guy, Linus Torvalds, used GCC to make his operating system (yes, Linux is an OS -- more on \
this later). He named it 'Linux' with a little help from his friends. Why doesn't he call it \
GNU/Linux? Because he wrote it, with more help from his friends, not you. You named your stuff, I \
named my stuff -- including the software I wrote using GCC -- and Linus named his stuff. The \
proper name is Linux because Linus Torvalds says so. Linus has spoken. Accept his authority. To do \
otherwise is to become a nag. You don't want to be known as a nag, do you?

(An operating system) != (a distribution). Linux is an operating system. By my definition, an \
operating system is that software which provides and limits access to hardware resources on a \
computer. That definition applies wherever you see Linux in use. However, Linux is usually \
distributed with a collection of utilities and applications to make it easily configurable as a \
desktop system, a server, a development box, or a graphics workstation, or whatever the user \
needs. In such a configuration, we have a Linux (based) distribution. Therein lies your strongest \
argument for the unwieldy title 'GNU/Linux' (when said bundled software is largely from the FSF). \
Go bug the distribution makers on that one. Take your beef to Red Hat, Mandrake, and Slackware. At \
least there you have an argument. Linux alone is an operating system that can be used in various \
applications without any GNU software whatsoever. Embedded applications come to mind as an obvious \
example.";

const UN_INTERJECTION_2: &str = "Next, even if we limit the GNU/Linux title to the GNU-based Linux \
distributions, we run into another obvious problem. XFree86 may well be more important to a \
particular Linux installation than the sum of all the GNU contributions. More properly, shouldn't \
the distribution be called XFree86/Linux? Or, at a minimum, XFree86/GNU/Linux? Of course, it would \
be rather arbitrary to draw the line there when many other fine contributions go unlisted. Yes, I \
know you've heard this one before. Get used to it. You'll keep hearing it until you can cleanly \
counter it.

You seem to like the lines-of-code metric. There are many lines of GNU code in a typical Linux \
distribution. You seem to suggest that (more LOC) == (more important). However, I submit to you \
that raw LOC numbers do not directly correlate with importance. I would suggest that clock cycles \
spent on code is a better metric. For example, if my system spends 90% of its time executing \
XFree86 code, XFree86 is probably the single most important collection of code on my system. Even \
if I loaded ten times as many lines of useless bloatware on my system and I never excuted that \
bloatware, it certainly isn't more important code than XFree86. Obviously, this metric isn't \
perfect either, but LOC really, really sucks. Please refrain from using it ever again in \
supporting any argument.

Last, I'd like to point out that we Linux and GNU users shouldn't be fighting among ourselves over \
naming other people's software. But what the heck, I'm in a bad mood now. I think I'm feeling \
sufficiently obnoxious to make the point that GCC is so very famous and, yes, so very useful only \
because Linux was developed. In a show of proper respect and gratitude, shouldn't you and everyone \
refer to GCC as 'the Linux compiler'? Or at least, 'Linux GCC'? Seriously, where would your \
masterpiece be without Linux? Languishing with the HURD?";