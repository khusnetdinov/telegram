use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::forum_topic_reopened::ForumTopicReopened;
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForumTopicReopenedMessage {
    pub forum_topic_reopened: ForumTopicReopened,
}

impl From<Inner> for ForumTopicReopenedMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            forum_topic_reopened,
            ..
        } = inner;

        Self {
            forum_topic_reopened: forum_topic_reopened.unwrap(),
        }
    }
}
