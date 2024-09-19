use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::bot_short_description::BotShortDescription as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct BotShortDescription {
    pub short_description: String,
}
