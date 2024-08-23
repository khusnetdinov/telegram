use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::bot_description::BotDescription as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct BotDescription {
    pub description: String,
}
