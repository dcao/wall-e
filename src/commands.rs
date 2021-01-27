use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "wall-e 2021-01-26").await?;

    Ok(())
}
