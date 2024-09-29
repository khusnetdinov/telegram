use crate::feature::web_app::WebAppInfo;
use crate::structs::reply_markups::keyboard_buttons::keyboard_button_poll_type::KeyboardButtonPollType;
use crate::structs::reply_markups::keyboard_buttons::keyboard_button_request_chat::KeyboardButtonRequestChat;
use crate::structs::reply_markups::keyboard_buttons::keyboard_button_request_users::KeyboardButtonRequestUsers;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::keyboard_button::KeyboardButton as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct KeyboardButton {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_users: Option<KeyboardButtonRequestUsers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_chat: Option<KeyboardButtonRequestChat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<KeyboardButtonPollType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
}
