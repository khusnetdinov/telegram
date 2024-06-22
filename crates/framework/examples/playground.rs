#![allow(dead_code)]
use telegram_framework::bots_api::BotsApi;
use telegram_framework::enums::message_kind::MessageKind;
use telegram_framework::enums::update_kind::UpdateKind;
use telegram_framework::storages::memory::MemoryStorage;
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

#[derive(Debug, Clone)]
pub enum States {
    Start,
    Help,
    Username,
    Text,
    Dice,
}

async fn schema(update: Update) -> Result<(), Box<dyn std::error::Error>> {
    match update.dispatch() {
        UpdateKind::Message(message) => match message.dispatch() {
            MessageKind::Text(text_message) => {
                println!("{:#?}", text_message);
            }
            MessageKind::Command(command_message) => match Commands::dispatch(command_message) {
                Some(Commands::Help) => {
                    println!("{:#?}", command_message);
                }
                Some(Commands::Username) => {
                    println!("{:#?}", command_message);
                }
                Some(Commands::Dice) => {
                    println!("{:#?}", command_message);
                }
                _ => println!("Commmand::Unexpected"),
            },
            MessageKind::Unexpected(_) | _ => {}
        },
        UpdateKind::Unexpected(_) | _ => {}
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = MemoryStorage::<States>::new();
    let bots_api = BotsApi::from_env().await?;

    bots_api.commands(Commands::config()).await?;
    bots_api.pooling(schema).await?;

    Ok(())
}
