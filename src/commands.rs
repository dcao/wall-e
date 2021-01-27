use serenity::framework::standard::{
    macros::{command, help},
    Args, CommandGroup, CommandResult, HelpOptions, help_commands,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::{collections::HashSet, process::Command};

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
#[num_args(1)]
async fn run(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg = args.quoted().single::<String>().unwrap();
    let output = Command::new("python")
        .arg("-c")
        .arg(&arg)
        .output()?;

    let s1 = String::from_utf8(output.stdout)?;
    let s2 = String::from_utf8(output.stderr)?;

    msg.channel_id.say(&ctx.http, format!(r"
```
--- stdout:
{}
--- stderr:
{}
```
", s1, s2)).await?;

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
