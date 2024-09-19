use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::bot_name::BotName as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct BotName {
    pub name: String,
}
