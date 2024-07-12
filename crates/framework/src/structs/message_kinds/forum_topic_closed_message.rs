use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::forum_topic_closed::ForumTopicClosed;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForumTopicClosedMessage {
    pub forum_topic_closed: ForumTopicClosed,
}

impl From<Message> for ForumTopicClosedMessage {
    fn from(remote: Message) -> Self {
        let Message {
            forum_topic_closed, ..
        } = remote;

        Self {
            forum_topic_closed: forum_topic_closed.unwrap(),
        }
    }
}
