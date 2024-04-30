use telegram_bots_api::api::params::delete_my_commands::DeleteMyCommands;
use telegram_bots_api::api::params::get_my_commands::GetMyCommands;
use telegram_bots_api::api::params::set_my_commands::SetMyCommands;
use telegram_framework::bots_api::BotsApi;
use telegram_framework::config::Config;
use telegram_macros::BotCommands;

fn main() {
    let config = Config::new();
    let bots_api = BotsApi::from(config);

    // #[derive(Debug, BotCommands)]
    // #[command(description = "Supported commands:", language_code = "ru", scope = "default")]
    // #[command(description = "all_chat_administrators",  scope = "all_chat_administrators")]
    // #[command(description = "all_group_chats", scope = "all_group_chats")]
    // #[command(description = "all_private_chats", scope = "all_private_chats")]
    // #[command(description = "chat", scope = "chat", chat_id = "-1002109487074")]
    // #[command(description = "chat_administrators", scope = "chat_administrators", chat_id = "-1002109487074")]
    // #[command(description = "chat_member", scope = "chat_member", chat_id = "-1002109487074", user_id = "6591790550")]
    // enum DefaultCommands {
    //     #[command(description = "help command")]
    //     Help,
    //     #[command(description = "Show you user name")]
    //     Username,
    // }

    // impl DefaultCommands {
    //     fn set_params() -> SetMyCommands {
    //         SetMyCommands {
    //             commands: vec![],
    //             scope: None,
    //             language_code: None
    //         }
    //     }
    //
    //     fn get_params() -> GetMyCommands {
    //         GetMyCommands {
    //             scope: None,
    //             language_code: None
    //         }
    //     }
    //
    //     fn delete_params() -> DeleteMyCommands {
    //         DeleteMyCommands {
    //             scope: None,
    //             language_code: None
    //         }
    //     }
    // }
    //
    // dbg!(DefaultCommands::set_params());

    bots_api.pooling(false, move |_update| {});
}
