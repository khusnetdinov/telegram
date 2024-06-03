use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::forum_topic_created::ForumTopicCreated;
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForumTopicCreatedMessage {
    pub forum_topic_created: ForumTopicCreated,
}

impl From<Inner> for ForumTopicCreatedMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            forum_topic_created,
            ..
        } = inner;

        Self {
            forum_topic_created: forum_topic_created.unwrap(),
        }
    }
}
