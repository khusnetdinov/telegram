use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::video_chat_started::VideoChatStarted as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct VideoChatStarted {}

impl From<Message> for VideoChatStarted {
    fn from(remote: Message) -> Self {
        let Message {
            video_chat_started, ..
        } = remote;

        Self::from(video_chat_started.unwrap())
    }
}
