use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::general_forum_topic_unhidden::GeneralForumTopicUnhidden as Remote;
use telegram_bots_api::api::structs::message::Message;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct GeneralForumTopicUnhidden {}

impl From<Message> for GeneralForumTopicUnhidden {
    fn from(remote: Message) -> Self {
        let Message {
            general_forum_topic_unhidden,
            ..
        } = remote;

        Self::from(general_forum_topic_unhidden.unwrap())
    }
}
