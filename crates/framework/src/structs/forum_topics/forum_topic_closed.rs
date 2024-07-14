use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::forum_topic_closed::ForumTopicClosed as Remote;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForumTopicClosed {}

impl From<Remote> for ForumTopicClosed {
    fn from(_remote: Remote) -> Self {
        Self {}
    }
}

impl From<Message> for ForumTopicClosed {
    fn from(remote: Message) -> Self {
        let Message {
            forum_topic_closed, ..
        } = remote;

        Self::from(forum_topic_closed.unwrap())
    }
}
