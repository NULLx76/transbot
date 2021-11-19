use std::env;

use serenity::{
    async_trait,
    http::CacheHttp,
    model::{
        channel::Message,
        gateway::Ready,
        id::{EmojiId, GuildId},
    },
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let emojis = GuildId(483224125125427200).emojis(&ctx.http).await.unwrap();
        let fr00d = emojis.into_iter().find(|e| e.name == "fr00d").unwrap();

        if msg.content.contains("trans") {
            if let Err(why) = msg.react(&ctx.http(), fr00d).await {
                eprintln!("Error adding reaction: {:?}", why)
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
