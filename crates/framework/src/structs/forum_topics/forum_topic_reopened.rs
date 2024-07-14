use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::forum_topic_reopened::ForumTopicReopened as Remote;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForumTopicReopened {}

impl From<Remote> for ForumTopicReopened {
    fn from(_remote: Remote) -> Self {
        Self {}
    }
}

impl From<Message> for ForumTopicReopened {
    fn from(remote: Message) -> Self {
        let Message {
            forum_topic_reopened: Some(forum_topic_reopened),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(forum_topic_reopened)
    }
}
