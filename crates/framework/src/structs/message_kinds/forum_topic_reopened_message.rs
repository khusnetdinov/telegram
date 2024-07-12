use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::forum_topic_reopened::ForumTopicReopened;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForumTopicReopenedMessage {
    pub forum_topic_reopened: ForumTopicReopened,
}

impl From<Message> for ForumTopicReopenedMessage {
    fn from(remote: Message) -> Self {
        let Message {
            forum_topic_reopened,
            ..
        } = remote;

        Self {
            forum_topic_reopened: forum_topic_reopened.unwrap(),
        }
    }
}
