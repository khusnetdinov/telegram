use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::params::send_message::SendMessage;
use telegram_bots_api::api::requests::sync::Requests;
use telegram_bots_api::clients::sync::Sync;

use telegram_framework::bots_api::BotsApi;
use telegram_framework::enums::messages::{CommandMessage, Messages, TextMessage};
use telegram_framework::structs::update::Update;
use telegram_macros::BotCommands;

fn main() {
    let mut bots_api = BotsApi::new();

    #[allow(dead_code)]
    #[derive(Debug, BotCommands)]
    #[command(scope = "default")]
    // #[command(scope = "all_chat_administrators")]
    // #[command(scope = "all_group_chats")]
    // #[command(scope = "all_private_chats", language_code = "ru")]
    // #[command(scope = "chat", chat_id = "-1002109487074", language_code = "ru")]
    // #[command(scope = "chat_administrators", chat_id = "-1002109487074", language_code = "ru")]
    // #[command(scope = "chat_member", chat_id = "-1002109487074", user_id = "6591790550", language_code = "ru")]
    enum DefaultCommands {
        #[command(description = "help command description")]
        Help,
        #[command(description = "enter username")]
        Username

    }

    DefaultCommands::configure(&bots_api);

    bots_api.pooling(true, move |update: &Update, client: &Sync| {
        match update.dispatch() {
            Some(Messages::Text(message)) => {
                let TextMessage { chat, text, .. } = message;

                client
                    .send_message(&SendMessage {
                        chat_id: ChatUId::from(chat.id),
                        text: format!("You have entered text: #{}", text),
                        ..SendMessage::default()
                    })
                    .unwrap();
            }
            Some(Messages::Command(message)) => {
                let CommandMessage { chat, .. } = &message;

                match DefaultCommands::dispatch(&message) {
                    Some(DefaultCommands::Help) => {
                        client
                            .send_message(&SendMessage {
                                chat_id: ChatUId::from(chat.id),
                                text: String::from("You have entered /help"),
                                ..SendMessage::default()
                            })
                            .unwrap();
                    }
                    Some(DefaultCommands::Username) => {
                        client
                            .send_message(&SendMessage {
                                chat_id: ChatUId::from(chat.id),
                                text: String::from("You have entered /username"),
                                ..SendMessage::default()
                            })
                            .unwrap();
                    }
                    _ => {
                        client
                            .send_message(&SendMessage {
                                chat_id: ChatUId::from(chat.id),
                                text: String::from("You have entered not valid command"),
                                ..SendMessage::default()
                            })
                            .unwrap();
                    }
                }
            }
            _ => {
                let chat_id = update.message.as_deref().unwrap().chat.id;

                client
                    .send_message(&SendMessage {
                        chat_id: ChatUId::from(chat_id),
                        text: String::from("None"),
                        ..SendMessage::default()
                    })
                    .unwrap();
            }
        }
    })
}
