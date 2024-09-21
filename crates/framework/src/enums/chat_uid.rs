use crate::structs::chats::chat_id::ChatId;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::chat_uid::ChatUId as Remote;
use telegram_macros::{FromRemoteEnum, IntoRemoteEnum};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum, IntoRemoteEnum)]
#[serde(untagged)]
pub enum ChatUId {
    I64(ChatId),
    Username(String),
}

impl From<i64> for ChatUId {
    fn from(chat_id: i64) -> Self {
        Self::I64(ChatId::from(chat_id))
    }
}

impl From<&str> for ChatUId {
    fn from(username: &str) -> Self {
        Self::Username(String::from(username))
    }
}

impl From<String> for ChatUId {
    fn from(string: String) -> Self {
        match string.parse::<i64>() {
            Ok(chat_id) => Self::I64(ChatId::from(chat_id)),
            Err(_) => Self::Username(string),
        }
    }
}

impl Default for ChatUId {
    fn default() -> Self {
        Self::I64(ChatId(0))
    }
}
