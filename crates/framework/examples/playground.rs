#![allow(dead_code)]

use telegram_framework::feature::bots_api::*;
use telegram_framework::feature::chat::*;
use telegram_framework::feature::commands::*;
use telegram_framework::feature::dice::*;
use telegram_framework::feature::media::media_group::*;
use telegram_framework::feature::media::photo::*;
use telegram_framework::feature::pooling::*;

use telegram_framework::traits::features::message::Message;
use telegram_framework::traits::storage::Storage;

#[derive(Debug, BotCommands)]
#[command(scope = "default")]
// #[command(scope = "all_chat_administrators")]
// #[command(scope = "all_group_chats")]
// #[command(scope = "all_private_chats", language_code = "ru")]
// #[command(scope = "chat", chat_id = "-1002109487074", language_code = "ru")]
// #[command(scope = "chat_administrators", chat_id = "-1002109487074", language_code = "ru")]
// #[command(scope = "chat_member", chat_id = "-1002109487074", user_id = "6591790550", language_code = "ru")]
pub enum BotCommands {
    #[command(description = "help command description")]
    Help,
    #[command(description = "enter username")]
    Dice,
    #[command(description = "send photo")]
    Photo,
    #[command(description = "media group")]
    MediaGroup,
}

#[derive(Debug, Clone)]
pub enum States {
    Init,
}

async fn dispatch(
    bots_api: BotsApi,
    storage: Arc<Mutex<MemoryStorage<String>>>,
    update: Update,
) -> Result<(), Box<dyn std::error::Error>> {
    match update.dispatch() {
        Updates::Message(message) => match message.dispatch() {
            MessageKind::Command(command_message) => match BotCommands::dispatch(command_message) {
                Some(BotCommands::Help) => {
                    println!("{:#?}", command_message);
                }
                Some(BotCommands::Dice) => {
                    let options = DiceOptions {
                        message_effect_id: Some(String::from("5046589136895476101")),
                        ..Default::default()
                    };

                    bots_api
                        .send_dice(ChatUId::from(message.chat.id), Some(Emoji::Darts), options)
                        .await?;
                }
                Some(BotCommands::Photo) => {
                    let photo = FileInput::from("https://248006.selcdn.ru/main/iblock/73d/73da4a4a09e01c1a4b2f20d3a870ac62/f8c5806b72c401ebaa6a32a2a482a3d4.png".to_string());
                    let options = MediaOptions {
                        ..Default::default()
                    };

                    bots_api
                        .send_photo(ChatUId::from(message.chat.id), photo, options)
                        .await?;
                }
                Some(BotCommands::MediaGroup) => {
                    let photo = InputMedia::Photo(InputMediaPhoto {
                            kind: String::from("photo"),
                            media: String::from("AgACAgQAAxkDAAIFe2bDfFz2HfMVS53Mw4eibmQ-pRrrAAJ9rzEbVmCEUPbcHIfXrjbrAQADAgADcwADNQQ"),
                            ..Default::default()
                        });

                    let media = vec![photo.clone(), photo.clone(), photo.clone()];
                    let options = MediaOptions {
                        ..Default::default()
                    };

                    bots_api
                        .send_media_group(ChatUId::from(message.chat.id), media, options)
                        .await?;
                }
                _ => println!("Command::Unexpected"),
            },
            MessageKind::Text(text_message) => {
                let storage = Arc::clone(&storage);
                let mut storage = storage.lock().await;

                storage.set(message.chat.id, text_message.text.to_string());

                println!("State: {:?}", storage);

                let options = telegram_framework::structs::messages::options::Options {
                    message_effect_id: Some(String::from("5046589136895476101")),
                    ..Default::default()
                };

                bots_api
                    .send_chat_action(
                        ChatUId::from(message.chat.id),
                        ChatAction::Typing,
                        None,
                        None,
                    )
                    .await?;

                bots_api
                    .send_message(
                        ChatUId::from(message.chat.id),
                        format!("Text: {}", text_message.text),
                        options,
                    )
                    .await?;
            }
            MessageKind::Unexpected(_) | _ => {}
        },
        Updates::Unexpected(_) | _ => {}
    }

    dbg!(storage);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bots_api = BotsApi::from_env().await?;
    let storage = Arc::new(Mutex::new(MemoryStorage::<String>::new()));

    bots_api.commands(BotCommands::config()).await?;
    bots_api.pooling(None, storage, dispatch).await?;

    Ok(())
}
