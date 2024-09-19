use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::bot_command::BotCommand as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}
