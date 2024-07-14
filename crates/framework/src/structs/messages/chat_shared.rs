use crate::structs::media::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_shared::ChatShared as Remote;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatShared {
    pub request_id: i64,
    pub chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
}
impl From<Remote> for ChatShared {
    fn from(remote: Remote) -> Self {
        Self {
            request_id: remote.request_id,
            chat_id: remote.chat_id,
            title: remote.title,
            username: remote.username,
            // TODO: #[remote(option, map, into)]
            photo: remote
                .photo
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
        }
    }
}

impl From<Message> for ChatShared {
    fn from(remote: Message) -> Self {
        let Message { chat_shared, .. } = remote;

        Self::from(chat_shared.unwrap())
    }
}
