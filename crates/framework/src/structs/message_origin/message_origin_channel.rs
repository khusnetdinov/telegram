use crate::structs::chat::Chat;
use crate::structs::messages::message_id::MessageId;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message_origin_channel::MessageOriginChannel as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct MessageOriginChannel {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub date: i64,
    pub chat: Chat,
    pub message_id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
}
