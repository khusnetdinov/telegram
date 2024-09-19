use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::forum_topic::ForumTopic as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct ForumTopic {
    pub message_thread_id: i64,
    pub name: String,
    pub icon_color: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}
