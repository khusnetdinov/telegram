#![allow(dead_code)]
use telegram_framework::prelude::*;
use telegram_framework::storages::memory::MemoryStorage;
use telegram_framework::traits::bots_api::Pooler;

#[derive(Debug, BotCommands)]
#[command(scope = "default")]
// #[command(scope = "all_chat_administrators")]
// #[command(scope = "all_group_chats")]
// #[command(scope = "all_private_chats", language_code = "ru")]
// #[command(scope = "chat", chat_id = "-1002109487074", language_code = "ru")]
// #[command(scope = "chat_administrators", chat_id = "-1002109487074", language_code = "ru")]
// #[command(scope = "chat_member", chat_id = "-1002109487074", user_id = "6591790550", language_code = "ru")]
pub enum Commands {
    #[command(description = "help command description")]
    Help,
    #[command(description = "enter username")]
    Username,
}

#[derive(Debug, Clone)]
pub enum States {
    Start,
    Help,
    Username,
    Text,
}

fn main() {
    let bots_api = BotsApi::from_env();
    let state = MemoryStorage::<States>::new();

    // TODO: rework configs
    // bots_api.commands(Commands
    // bots_api.set_commands(Commands::params);
    // bots_api.delete_commands(Commands::params);
    Commands::configure(&bots_api);

    bots_api.pooling(
        true,
        move |_bots_api: &BotsApi, update: Update| match update.dispatch() {
            UpdateKind::Message(message) => match message.dispatch() {
                MessageKind::Text(text_message) => {
                    println!("{:#?}", text_message);
                    state.set(message.chat.id, States::Text);
                    println!("{:#?}", state);
                }
                MessageKind::Command(command_message) => {
                    match Commands::dispatch(command_message) {
                        Some(Commands::Help) => {
                            println!("{:#?}", command_message);
                            state.set(message.chat.id, States::Help);
                            println!("{:#?}", state);
                        }
                        Some(Commands::Username) => {
                            println!("{:#?}", command_message);
                            state.set(message.chat.id, States::Username);
                            println!("{:#?}", state);
                        }
                        _ => println!("Commmand::Unexpected"),
                    }
                }
                MessageKind::Unexpected(_) | _ => {}
            },
            UpdateKind::Unexpected(_) | _ => {}
        },
    )
}
