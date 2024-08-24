use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::video_chat_started::VideoChatStarted as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct VideoChatStarted {}

impl From<Message> for VideoChatStarted {
    fn from(remote: Message) -> Self {
        let Message {
            video_chat_started, ..
        } = remote;

        Self::from(video_chat_started.unwrap())
    }
}
