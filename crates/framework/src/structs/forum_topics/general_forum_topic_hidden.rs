use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::general_forum_topic_hidden::GeneralForumTopicHidden as Remote;
use telegram_bots_api::api::structs::message::Message;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct GeneralForumTopicHidden {}

impl From<Message> for GeneralForumTopicHidden {
    fn from(remote: Message) -> Self {
        let Message {
            general_forum_topic_hidden,
            ..
        } = remote;

        Self::from(general_forum_topic_hidden.unwrap())
    }
}
