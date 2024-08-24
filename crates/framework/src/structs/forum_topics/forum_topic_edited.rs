use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::forum_topic_created::ForumTopicCreated as Remote;
use telegram_bots_api::api::structs::message::Message;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct ForumTopicCreated {
    pub name: String,
    pub icon_color: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

impl From<Message> for ForumTopicCreated {
    fn from(remote: Message) -> Self {
        let Message {
            forum_topic_created,
            ..
        } = remote;

        Self::from(forum_topic_created.unwrap())
    }
}
