use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::bot_short_description::BotShortDescription as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct BotShortDescription {
    pub short_description: String,
}
