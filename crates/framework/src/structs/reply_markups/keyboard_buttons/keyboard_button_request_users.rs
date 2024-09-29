use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::keyboard_button_request_users::KeyboardButtonRequestUsers as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct KeyboardButtonRequestUsers {
    pub request_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_premium: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_quantity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}
