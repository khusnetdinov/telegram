use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::general_forum_topic_unhidden::GeneralForumTopicUnhidden as Remote;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneralForumTopicUnhidden {}

impl From<Remote> for GeneralForumTopicUnhidden {
    fn from(_remote: Remote) -> Self {
        Self {}
    }
}

impl From<Message> for GeneralForumTopicUnhidden {
    fn from(remote: Message) -> Self {
        let Message {
            general_forum_topic_unhidden: Some(general_forum_topic_unhidden),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(general_forum_topic_unhidden)
    }
}
