use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::forum_topic_reopened::ForumTopicReopened as Remote;
use telegram_bots_api::api::structs::message::Message;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct ForumTopicReopened {}

impl From<Message> for ForumTopicReopened {
    fn from(remote: Message) -> Self {
        let Message {
            forum_topic_reopened,
            ..
        } = remote;

        Self::from(forum_topic_reopened.unwrap())
    }
}
