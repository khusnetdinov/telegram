use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::background_type_chat_theme::BackgroundTypeChatTheme as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct BackgroundTypeChatTheme {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub theme_name: String,
}
