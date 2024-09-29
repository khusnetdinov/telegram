use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::keyboard_button_poll_type::KeyboardButtonPollType as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct KeyboardButtonPollType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: Option<String>,
}
