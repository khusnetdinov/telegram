#![allow(dead_code)]
use telegram_framework::feature::bots_api::*;
use telegram_framework::feature::chat_actions::*;
use telegram_framework::feature::commands::*;
use telegram_framework::feature::contact::*;
use telegram_framework::feature::dice::*;
use telegram_framework::feature::game::*;
use telegram_framework::feature::photo::*;
use telegram_framework::feature::poll::*;
use telegram_framework::feature::pooling::*;

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
    #[command(description = "send contact")]
    Contact,
    #[command(description = "send game")]
    Game,
    #[command(description = "send poll")]
    Poll,
    #[command(description = "send photo")]
    Photo,
}

#[derive(Debug, Clone)]
pub enum States {
    Init,
}

async fn dispatch(
    bots_api: BotsApi,
    storage: Arc<MemoryStorage<States>>,
    update: Update,
) -> Result<(), Box<dyn std::error::Error>> {
    match update.dispatch() {
        Updates::Message(message) => match message.dispatch() {
            Messages::Command(command_message) => match BotCommands::dispatch(command_message) {
                Some(BotCommands::Help) => {
                    println!("{:#?}", command_message);
                }
                Some(BotCommands::Dice) => {
                    let options = Options {
                        message_effect_id: Some(String::from("5046589136895476101")),
                        ..Default::default()
                    };

                    bots_api
                        .send_chat_action(message.chat.id, ChatAction::Typing, None)
                        .await?;
                    bots_api
                        .send_dice(message.chat.id, Some(Emoji::Darts), Some(options))
                        .await?;
                }
                Some(BotCommands::Contact) => {
                    let contact = Contact {
                        phone_number: String::from("+79001234567"),
                        first_name: String::from("FirstName"),
                        ..Default::default()
                    };

                    bots_api
                        .send_contact(message.chat.id, contact, None)
                        .await?;
                }
                Some(BotCommands::Game) => {
                    let error = bots_api
                        .send_game(message.chat.id, String::from("test"), None)
                        .await
                        .unwrap_err();

                    println!("Error: {:#?}", error);
                }
                Some(BotCommands::Poll) => {
                    let poll_options = PollOptions {
                        ..Default::default()
                    };

                    let input_poll_options = vec![
                        InputPollOption {
                            text: Some("Ответ 1".to_string()),
                            text_parse_mode: Some("".to_string()),
                            ..Default::default()
                        },
                        InputPollOption {
                            text: Some("Ответ 2".to_string()),
                            text_parse_mode: Some("".to_string()),
                            ..Default::default()
                        },
                    ];

                    bots_api
                        .send_poll(
                            message.chat.id,
                            String::from("Вопрос?"),
                            String::from("regular"),
                            poll_options,
                            input_poll_options,
                            None,
                        )
                        .await?;
                }
                Some(BotCommands::Photo) => {
                    let photo = FileInput::from("https://248006.selcdn.ru/main/iblock/73d/73da4a4a09e01c1a4b2f20d3a870ac62/f8c5806b72c401ebaa6a32a2a482a3d4.png".to_string());
                    let media_options = MediaOptions {
                        ..Default::default()
                    };

                    bots_api
                        .send_photo(message.chat.id, photo, media_options, None)
                        .await?;
                }
                _ => println!("Command::Unexpected"),
            },
            // MessageKind::Text(text_message) => {
            //     let options = SendOptions {
            //         message_effect_id: Some(String::from("5046589136895476101")),
            //         ..Default::default()
            //     };

            //     bots_api
            //         .send_chat_action(message.chat.id, ChatAction::Typing, None)
            //         .await?;

            //     sleep(Duration::from_secs(1)).await;

            //     bots_api
            //         .send_message(
            //             message.chat.id,
            //             format!("Text: {}", text_message.text),
            //             Some(options),
            //         )
            //         .await?;
            //
            Messages::Unexpected(_) | _ => {}
        },
        Updates::Unexpected(_) | _ => {}
    }

    dbg!(bots_api);
    dbg!(update);
    dbg!(storage);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bots_api = BotsApi::from_env().await?;
    let storage = MemoryStorage::<States>::new();

    bots_api.commands(BotCommands::config()).await?;
    bots_api.pooling(storage, dispatch).await?;

    Ok(())
}
