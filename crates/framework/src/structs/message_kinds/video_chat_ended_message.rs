use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::video_chat_ended::VideoChatEnded;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoChatEndedMessage {
    pub video_chat_ended: VideoChatEnded,
}

impl From<Message> for VideoChatEndedMessage {
    fn from(remote: Message) -> Self {
        let Message {
            video_chat_ended, ..
        } = remote;

        Self {
            video_chat_ended: video_chat_ended.unwrap(),
        }
    }
}
