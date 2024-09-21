use crate::enums::chat_uid::ChatUId;
use crate::structs::media::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_shared::ChatShared as Remote;
use telegram_bots_api::api::structs::message::Message;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct ChatShared {
    pub request_id: i64,
    pub chat_id: ChatUId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
}

impl From<Message> for ChatShared {
    fn from(remote: Message) -> Self {
        let Message { chat_shared, .. } = remote;

        Self::from(chat_shared.unwrap())
    }
}
