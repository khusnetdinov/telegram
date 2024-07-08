use telegram_bots_api::api::structs::bot_command::BotCommand as Remote;
use telegram_macros::FromRemote;


#[derive(FromRemote)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}
