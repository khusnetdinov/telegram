use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;
use telegram_bots_api::api::structs::video_chat_started::VideoChatStarted as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct VideoChatStarted {}

impl From<IncomingMessage> for VideoChatStarted {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            video_chat_started, ..
        } = remote;

        Self::from(video_chat_started.unwrap())
    }
}
