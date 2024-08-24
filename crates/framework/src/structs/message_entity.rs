use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message_entity::MessageEntity as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
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

impl From<MessageEntity> for Remote {
    fn from(value: MessageEntity) -> Self {
        Self {
            kind: value.kind,
            offset: value.offset,
            length: value.length,
            url: value.url,
            // TODO: #[value(option, into)]
            user: value.user.map(|inner| inner.into()),
            language: value.language,
            custom_emoji_id: value.custom_emoji_id,
        }
    }
}
