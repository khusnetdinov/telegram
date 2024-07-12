use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::general_forum_topic_hidden::GeneralForumTopicHidden;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneralForumTopicHiddenMessage {
    pub general_forum_topic_hidden: GeneralForumTopicHidden,
}

impl From<Message> for GeneralForumTopicHiddenMessage {
    fn from(remote: Message) -> Self {
        let Message {
            general_forum_topic_hidden,
            ..
        } = remote;

        Self {
            general_forum_topic_hidden: general_forum_topic_hidden.unwrap(),
        }
    }
}
