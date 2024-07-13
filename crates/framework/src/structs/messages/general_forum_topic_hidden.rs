use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::general_forum_topic_hidden::GeneralForumTopicHidden as Remote;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneralForumTopicHidden {}

impl From<Remote> for GeneralForumTopicHidden {
    fn from(_from: Remote) -> Self {
        Self {}
    }
}

impl From<Message> for GeneralForumTopicHidden {
    fn from(remote: Message) -> Self {
        let Message {
            general_forum_topic_hidden: Some(general_forum_topic_hidden),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(general_forum_topic_hidden)
    }
}
