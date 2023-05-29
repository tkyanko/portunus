mod commands;
mod db;

use std::env;

use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::Context;
use serenity::prelude::*;

#[group]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("GUILD_ID must be set")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::add_key::register(command))
                .create_application_command(|command| commands::get_key::register(command))
                .create_application_command(|command| commands::list_keys::register(command))
        })
        .await;

        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let contents = match command.data.name.as_str() {
                "addkey" => commands::add_key::run(&command),
                "getkey" => commands::get_key::run(&command),
                "listkeys" => commands::list_keys::run(),
                _ => vec!["not implemented".to_string()],
            };

            let first_response = contents.first().expect("No message data");
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.ephemeral(true).content(first_response)
                        })
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
            if contents.len() > 1 {
                for content in contents.as_slice()[1..].iter().cloned() {
                    if let Err(why) = command
                        .create_followup_message(&ctx.http, |response| {
                            response.ephemeral(true).content(content)
                        })
                        .await
                    {
                        println!("Cannot respond to slash command: {}", why);
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("/")) // set the bot's prefix to "/"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN must be set");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
