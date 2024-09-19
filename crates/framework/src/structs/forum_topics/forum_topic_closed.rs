use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::forum_topic_closed::ForumTopicClosed as Remote;
use telegram_bots_api::api::structs::message::Message;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct ForumTopicClosed {}

impl From<Message> for ForumTopicClosed {
    fn from(remote: Message) -> Self {
        let Message {
            forum_topic_closed, ..
        } = remote;

        Self::from(forum_topic_closed.unwrap())
    }
}
