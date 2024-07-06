#![allow(dead_code)]
use telegram_framework::feature::bots_api::*;
use telegram_framework::feature::commands::*;
use telegram_framework::feature::pooling::*;
// use telegram_bots_api::api::structs::input_poll_option::InputPollOption;
// use telegram_framework::enums::chat_action::ChatAction;
// use telegram_framework::enums::emoji::Emoji;
// use telegram_framework::enums::message_kind::MessageKind;
// use telegram_framework::enums::update_kind::UpdateKind;
// use telegram_framework::structs::options::send_options::SendOptions;
// use telegram_framework::traits::kind_dispatcher::KindDispatcher;
// use telegram_framework::traits::sender::Sender;

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
    Username,
    #[command(description = "send dice")]
    Dice,
    #[command(description = "send game")]
    Game,
    #[command(description = "send poll")]
    Poll,
}

#[derive(Debug, Clone)]
pub enum States {
    Start,
    Help,
    Username,
    Text,
    Dice,
    Game,
    Poll,
}

async fn dispatch(
    bots_api: BotsApi,
    storage: Arc<MemoryStorage<States>>,
    update: Update,
) -> Result<(), Box<dyn std::error::Error>> {
    //     match update.dispatch() {
    //         UpdateKind::Message(message) => match message.dispatch() {
    //             MessageKind::Text(text_message) => {
    //                 let options = SendOptions {
    //                     message_effect_id: Some(String::from("5046589136895476101")),
    //                     ..Default::default()
    //                 };

    //                 bots_api
    //                     .send_chat_action(message.chat.id, ChatAction::Typing, None)
    //                     .await?;

    //                 sleep(Duration::from_secs(1)).await;

    //                 bots_api
    //                     .send_message(
    //                         message.chat.id,
    //                         format!("Text: {}", text_message.text),
    //                         Some(options),
    //                     )
    //                     .await?;
    //             }
    //             MessageKind::Command(command_message) => match Commands::dispatch(command_message) {
    //                 Some(Commands::Help) => {
    //                     println!("{:#?}", command_message);
    //                 }
    //                 Some(Commands::Username) => {
    //                     println!("{:#?}", command_message);
    //                 }
    //                 Some(Commands::Dice) => {
    //                     let options = SendOptions {
    //                         message_effect_id: Some(String::from("5046589136895476101")),
    //                         emoji: Some(Emoji::Darts),
    //                         ..Default::default()
    //                     };

    //                     bots_api.send_dice(message.chat.id, Some(options)).await?;
    //                 }
    //                 Some(Commands::Game) => {
    //                     if let Err(error) = bots_api
    //                         .send_game(message.chat.id, String::from("test"), None)
    //                         .await
    //                     {
    //                         println!("Error: {error:#?}");
    //                     };
    //                 }
    //                 Some(Commands::Poll) => {
    //                     let poll_options = vec![
    //                         InputPollOption {
    //                             text: Some("Ответ 1".to_string()),
    //                             text_parse_mode: Some("".to_string()),
    //                             ..Default::default()
    //                         },
    //                         InputPollOption {
    //                             text: Some("Ответ 2".to_string()),
    //                             text_parse_mode: Some("".to_string()),
    //                             ..Default::default()
    //                         },
    //                     ];

    //                     let options = SendOptions {
    //                         kind: Some(String::from("regular")),
    //                         ..Default::default()
    //                     };

    //                     bots_api
    //                         .send_poll(
    //                             message.chat.id,
    //                             String::from("Вопрос?"),
    //                             poll_options,
    //                             Some(options),
    //                         )
    //                         .await?;
    //                 }
    //                 _ => println!("Commmand::Unexpected"),
    //             },
    //             MessageKind::Unexpected(_) | _ => {}
    //         },
    //         UpdateKind::Unexpected(_) | _ => {}
    //     }

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
