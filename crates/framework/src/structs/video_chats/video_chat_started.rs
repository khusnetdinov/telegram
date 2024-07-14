use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::video_chat_started::VideoChatStarted as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoChatStarted {}

impl From<Remote> for VideoChatStarted {
    fn from(_remote: Remote) -> Self {
        Self {}
    }
}

impl From<Message> for VideoChatStarted {
    fn from(remote: Message) -> Self {
        let Message {
            video_chat_started: Some(video_chat_started),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(video_chat_started)
    }
}
