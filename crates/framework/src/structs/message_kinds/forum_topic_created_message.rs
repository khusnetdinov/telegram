use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::forum_topic_edited::ForumTopicEdited;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForumTopicEditedMessage {
    pub forum_topic_edited: ForumTopicEdited,
}

impl From<Message> for ForumTopicEditedMessage {
    fn from(remote: Message) -> Self {
        let Message {
            forum_topic_edited, ..
        } = remote;

        Self {
            forum_topic_edited: forum_topic_edited.unwrap(),
        }
    }
}
