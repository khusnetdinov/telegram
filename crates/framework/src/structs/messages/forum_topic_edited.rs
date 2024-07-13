use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::forum_topic_created::ForumTopicCreated as Remote;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForumTopicCreated {
    pub name: String,
    pub icon_color: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

impl From<Remote> for ForumTopicCreated {
    fn from(remote: Remote) -> Self {
        Self {
            name: remote.name,
            icon_color: remote.icon_color,
            icon_custom_emoji_id: remote.icon_custom_emoji_id,
        }
    }
}

impl From<Message> for ForumTopicCreated {
    fn from(remote: Message) -> Self {
        let Message {
            forum_topic_created: Some(forum_topic_created),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(forum_topic_created)
    }
}
