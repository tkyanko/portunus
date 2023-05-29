use crate::db::db::get_key;

use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;

pub fn run(interaction: &ApplicationCommandInteraction) -> Vec<String> {
    let title = interaction
        .data
        .options
        .get(0)
        .expect("Expected game name")
        .resolved
        .as_ref()
        .expect("Expected game object");

    let user_id = interaction.user.id.as_u64().to_string();

    if let CommandDataOptionValue::String(game_name) = title {
        match get_key(game_name, &user_id) {
            Ok(product_key) => vec![format!("Your key for {} is {}", game_name, product_key)],
            Err(error) => vec![error],
        }
    } else {
        vec!["Please provide a game name".to_string()]
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("getkey")
        .description("Get a key.")
        .create_option(|option| {
            option
                .name("game")
                .description("Name of the game.")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
