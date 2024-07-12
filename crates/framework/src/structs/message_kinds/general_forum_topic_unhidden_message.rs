use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::general_forum_topic_unhidden::GeneralForumTopicUnhidden;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneralForumTopicUnhiddenMessage {
    pub general_forum_topic_unhidden: GeneralForumTopicUnhidden,
}

impl From<Message> for GeneralForumTopicUnhiddenMessage {
    fn from(remote: Message) -> Self {
        let Message {
            general_forum_topic_unhidden,
            ..
        } = remote;

        Self {
            general_forum_topic_unhidden: general_forum_topic_unhidden.unwrap(),
        }
    }
}
