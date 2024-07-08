use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat::Chat as Remote;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,
}

impl From<Remote> for Chat {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            id: remote.id,
            title: remote.title,
            username: remote.username,
            first_name: remote.first_name,
            last_name: remote.last_name,
            is_forum: remote.is_forum,
        }
    }
}
