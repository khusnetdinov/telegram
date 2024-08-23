use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::bot_command::BotCommand as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}
