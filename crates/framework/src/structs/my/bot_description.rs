use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::bot_description::BotDescription as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct BotDescription {
    pub description: String,
}
