use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::forum_topic_created::ForumTopicCreated;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForumTopicCreatedMessage {
    pub forum_topic_created: ForumTopicCreated,
}

impl From<Message> for ForumTopicCreatedMessage {
    fn from(remote: Message) -> Self {
        let Message {
            forum_topic_created,
            ..
        } = remote;

        Self {
            forum_topic_created: forum_topic_created.unwrap(),
        }
    }
}
