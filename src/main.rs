use dotenv::dotenv;
use serenity::{
    async_trait,
    framework::standard::{
        macros::group,
        StandardFramework,
    },
    http::Http,
    model::{
        gateway::Ready,
    },
};
use std::{
    collections::HashSet,
    env,
};

use serenity::prelude::*;

mod commands;
use commands::*;
mod hooks;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(about)]
struct General;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .on_mention(Some(bot_id))
                .prefix(">")
                .delimiters(vec![" "])
                // Sets the bot's owners. These will be used for commands that
                // are owners only.
                .owners(owners)
        })
        .unrecognised_command(hooks::unknown_command)
        .on_dispatch_error(hooks::dispatch_error)
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    // The total number of shards to use. The "current shard number" of a
    // shard - that is, the shard it is assigned to - is indexed at 0,
    // while the total shard count is indexed at 1.
    //
    // This means if you have 5 shards, your total shard count will be 5, while
    // each shard will be assigned numbers 0 through 4.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
