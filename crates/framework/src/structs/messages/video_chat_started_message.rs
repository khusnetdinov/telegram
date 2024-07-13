use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::video_chat_started::VideoChatStarted;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoChatStartedMessage {
    pub video_chat_started: VideoChatStarted,
}

impl From<Message> for VideoChatStartedMessage {
    fn from(remote: Message) -> Self {
        let Message {
            video_chat_started, ..
        } = remote;

        Self {
            video_chat_started: video_chat_started.unwrap(),
        }
    }
}
