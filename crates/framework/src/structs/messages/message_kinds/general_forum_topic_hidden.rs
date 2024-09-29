use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::general_forum_topic_hidden::GeneralForumTopicHidden as Remote;
use telegram_bots_api::api::structs::message::Message as IncomingMessage;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct GeneralForumTopicHidden {}

impl From<IncomingMessage> for GeneralForumTopicHidden {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            general_forum_topic_hidden,
            ..
        } = remote;

        Self::from(general_forum_topic_hidden.unwrap())
    }
}
