use crate::enums::maybe_inaccessible_message::MaybeInaccessibleMessage;
use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::callback_query::CallbackQuery as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub chat_instance: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<MaybeInaccessibleMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_short_name: Option<String>,
}
