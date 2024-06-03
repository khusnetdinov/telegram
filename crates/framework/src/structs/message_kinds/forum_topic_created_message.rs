use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::forum_topic_edited::ForumTopicEdited;
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForumTopicEditedMessage {
    pub forum_topic_edited: ForumTopicEdited,
}

impl From<Inner> for ForumTopicEditedMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            forum_topic_edited, ..
        } = inner;

        Self {
            forum_topic_edited: forum_topic_edited.unwrap(),
        }
    }
}
