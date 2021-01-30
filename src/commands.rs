use lipsum::MarkovChain;
use rand::rngs::ThreadRng;
use serenity::framework::standard::{
    help_commands,
    macros::{command, help},
    Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::{
    cell::RefCell,
    collections::HashSet,
    process::Command,
    time::{Duration, Instant},
};

pub const CORPUS: &'static str = include_str!(corpus.txt");

thread_local! {
    static CHAIN: RefCell<MarkovChain<'static, ThreadRng>> = {
        let mut chain = MarkovChain::new();
        chain.learn(&CORPUS);
        RefCell::new(chain)
    }
}

fn gen_string(n: usize) -> String {
    CHAIN.with(|cell| {
        let mut chain = cell.borrow_mut();
        chain.generate(n)
    })
}

#[command]
#[description("prints info about the bot")]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            "wall-e 2021-01-26 | https://github.com/dcao/wall-e",
        )
        .await?;

    Ok(())
}

#[command]
#[description("do a typing test!")]
async fn typing_test(ctx: &Context, msg: &Message) -> CommandResult {
    let wc = 30;
    let tt = gen_string(wc);

    let _ = msg.channel_id.say(&ctx.http, &tt).await;

    let pre = Instant::now();

    if let Some(answer) = &msg
        .channel_id
        .await_reply(&ctx)
        .timeout(Duration::from_secs(60))
        .author_id(msg.author.id)
        .await
    {
        if answer.content == tt {
            let dur = pre.elapsed().as_secs();
            let _ = answer
                .reply(ctx, format!("wpm: {}", (wc as f64) / (dur as f64) * 60.0))
                .await;
        } else {
            let _ = answer
                .reply(ctx, "strings didn't match. try `>typing_test` again!")
                .await;
        }
    } else {
        let _ = msg
            .reply(
                ctx,
                "nothing received after a minute. try `>typing_test` again!",
            )
            .await;
    };

    Ok(())
}

#[command]
#[num_args(1)]
async fn run(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg = args.quoted().single::<String>().unwrap();
    let output = Command::new("python").arg("-c").arg(&arg).output()?;

    let s1 = String::from_utf8(output.stdout)?;
    let s2 = String::from_utf8(output.stderr)?;

    msg.channel_id
        .say(
            &ctx.http,
            format!(
                r"
```
--- stdout:
{}
--- stderr:
{}
```
",
                s1, s2
            ),
        )
        .await?;

    Ok(())
}

#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
