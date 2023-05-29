use crate::db::db::list_keys;

use serenity::builder::CreateApplicationCommand;
use tabled::{builder::Builder, settings::Style};

pub fn run() -> Vec<String> {
    match list_keys() {
        Ok(key_list) => {
            let mut builder = Builder::default();
            for game_key_pair in key_list {
                let row = vec![game_key_pair.title, game_key_pair.count.to_string()];
                builder.push_record(row);
            }
            builder.set_header(["Game", "Keys"]);
            let mut table = builder.build();
            table.with(Style::markdown());
            let table_string = table.to_string();
            let table_rows = table_string.split('\n');
            let mut message_contents: Vec<String> = Vec::new();
            let mut char_count = 0;
            let mut single_message_contents = "".to_string();
            for row in table_rows {
                if char_count < 1800 {
                    single_message_contents += &*format!("{}\n", row);
                    char_count += row.chars().count();
                } else {
                    char_count = 0;
                    message_contents.push(format!("```\n{}\n```", single_message_contents));
                    single_message_contents = "".to_string();
                }
            }
            message_contents.push(format!("```\n{}\n```", single_message_contents));
            message_contents
        }
        Err(error) => vec![error],
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("listkeys")
        .description("List games with available keys.")
}
