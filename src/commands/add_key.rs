use crate::db::db::add_key;

use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;

pub fn run(interaction: &ApplicationCommandInteraction) -> Vec<String> {
    let name = interaction
        .data
        .options
        .get(0)
        .expect("Expected game name")
        .resolved
        .as_ref()
        .expect("Expected game object");

    let key = interaction
        .data
        .options
        .get(1)
        .expect("Expected product key")
        .resolved
        .as_ref()
        .expect("Expected product key");

    let user_id = interaction.user.id.as_u64().to_string();

    if let (
        CommandDataOptionValue::String(game_name),
        CommandDataOptionValue::String(product_key),
    ) = (name, key)
    {
        add_key(game_name, product_key, &user_id);
        vec![format!("Thanks for adding a key for {}", game_name)]
    } else {
        vec!["Please provide a game name and product key".to_string()]
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("addkey")
        .description("Add a key.")
        .create_option(|option| {
            option
                .name("game")
                .description("Name of the game.")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("key")
                .description("The product key.")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("store")
                .description("Store the product key can be redeemed on. Default: Steam")
                .kind(CommandOptionType::String)
                .required(false)
        })
}
