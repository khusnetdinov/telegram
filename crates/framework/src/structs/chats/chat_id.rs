use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_id::ChatId as Remote;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatId(pub i64);

impl From<i64> for ChatId {
    fn from(chat_id: i64) -> Self {
        Self(chat_id)
    }
}

impl From<Remote> for ChatId {
    fn from(value: Remote) -> Self {
        Self(value.0)
    }
}

impl From<ChatId> for Remote {
    fn from(value: ChatId) -> Self {
        Self(value.0)
    }
}
