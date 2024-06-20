#![allow(dead_code)]
use telegram_framework::bots_api::BotsApi;
use telegram_framework::enums::message_kind::MessageKind;
use telegram_framework::enums::update_kind::UpdateKind;
use telegram_framework::structs::update::Update;
use telegram_framework::traits::bots_api::Commander;
use telegram_framework::traits::bots_api::Pooler;
use telegram_framework::traits::dispatcher::Dispatcher;
use telegram_framework::traits::params::EnumParams;
use telegram_macros::BotCommands;

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
    #[command(description = "send dice")]
    Dice,
}

// #[derive(Debug, Clone)]
// pub enum States {
//     Start,
//     Help,
//     Username,
//     Text,
//     Dice,
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let state = MemoryStorage::<States>::new();
    let bots_api = BotsApi::from_env().await?;

    bots_api.commands(Commands::config()).await?;
    bots_api
        .pooling(|update: Update| match update.dispatch() {
            UpdateKind::Message(message) => match message.dispatch() {
                MessageKind::Text(text_message) => {
                    println!("{:#?}", update);
                    println!("{:#?}", text_message);
                    // state.set(message.chat.id, States::Text);
                    // println!("{:#?}", state);
                }
                MessageKind::Command(command_message) => {
                    match Commands::dispatch(command_message) {
                        Some(Commands::Help) => {
                            println!("{:#?}", update);
                            println!("{:#?}", command_message);
                            // state.set(message.chat.id, States::Help);
                            // println!("{:#?}", state);
                        }
                        Some(Commands::Username) => {
                            println!("{:#?}", update);
                            println!("{:#?}", command_message);
                            // state.set(message.chat.id, States::Username);
                            // println!("{:#?}", state);
                        }
                        Some(Commands::Dice) => {
                            println!("{:#?}", update);
                            println!("{:#?}", command_message);
                            // state.set(message.chat.id, States::Dice);
                            // println!("{:#?}", state);
                        }
                        _ => println!("Commmand::Unexpected"),
                    }
                }
                MessageKind::Unexpected(_) | _ => {}
            },
            UpdateKind::Unexpected(_) | _ => {}
        })
        .await?;

    Ok(())
}
