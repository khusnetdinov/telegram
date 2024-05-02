use telegram_framework::bots_api::BotsApi;
use telegram_framework::structs::update::Update;
use telegram_macros::BotCommands;

fn main() {
    let bots_api = BotsApi::new();

    #[allow(dead_code)]
    #[derive(Debug, BotCommands)]
    // #[command(scope = "default")]
    // #[command(scope = "all_chat_administrators")]
    // #[command(scope = "all_group_chats")]
    // #[command(scope = "all_private_chats", language_code = "ru")]
    // #[command(scope = "chat", chat_id = "-1002109487074", language_code = "ru")]
    // #[command(scope = "chat_administrators", chat_id = "-1002109487074", language_code = "ru")]
    #[command(
        scope = "chat_member",
        chat_id = "-1002109487074",
        user_id = "6591790550",
        language_code = "ru"
    )]
    enum DefaultCommands {
        #[command(description = "help command description")]
        Help,
        #[command(description = "Show you user name")]
        Username,
    }

    bots_api.pooling(true, move |update: Update| {
        println!("{update:?}");
    })
}
