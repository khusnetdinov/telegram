use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::forum_topic_closed::ForumTopicClosed;
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForumTopicClosedMessage {
    pub forum_topic_closed: ForumTopicClosed,
}

impl From<Inner> for ForumTopicClosedMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            forum_topic_closed, ..
        } = inner;

        Self {
            forum_topic_closed: forum_topic_closed.unwrap(),
        }
    }
}
