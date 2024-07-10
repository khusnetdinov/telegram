use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message_entity::MessageEntity as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageEntity {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub offset: i64,
    pub length: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
}
impl From<Remote> for MessageEntity {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            offset: remote.offset,
            length: remote.length,
            url: remote.url,
            // TODO: #[remote(option)]
            user: remote.user.map(|inner| inner.into()),
            language: remote.language,
            custom_emoji_id: remote.custom_emoji_id,
        }
    }
}
