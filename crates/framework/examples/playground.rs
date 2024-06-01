use telegram_framework::enums::update_kind::UpdateKind;
use telegram_framework::prelude::*;

#[allow(dead_code)]
#[derive(Debug, BotCommands)]
#[command(scope = "default")]
// #[command(scope = "all_chat_administrators")]
// #[command(scope = "all_group_chats")]
// #[command(scope = "all_private_chats", language_code = "ru")]
// #[command(scope = "chat", chat_id = "-1002109487074", language_code = "ru")]
// #[command(scope = "chat_administrators", chat_id = "-1002109487074", language_code = "ru")]
// #[command(scope = "chat_member", chat_id = "-1002109487074", user_id = "6591790550", language_code = "ru")]
pub enum DefaultCommands {
    #[command(description = "help command description")]
    Help,
    #[command(description = "enter username")]
    Username,
}

fn main() {
    let bots_api = BotsApi::from_env();
    DefaultCommands::configure(&bots_api);

    bots_api.pooling(
        true,
        move |_bots_api: &BotsApi, update: Update| match update.kind {
            UpdateKind::Message(message) => {
                println!("UpdateKind::Message: {:#?}", message);
            }
            UpdateKind::Unexpected | _ => {}
        },
    );
}
