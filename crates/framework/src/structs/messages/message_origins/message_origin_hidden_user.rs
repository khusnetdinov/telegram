use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message_origin_hidden_user::MessageOriginHiddenUser as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct MessageOriginHiddenUser {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub date: i64,
    pub sender_user_name: String,
}
