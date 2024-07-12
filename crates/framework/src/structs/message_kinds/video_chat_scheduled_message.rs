use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::video_chat_scheduled::VideoChatScheduled;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoChatScheduledMessage {
    pub video_chat_scheduled: VideoChatScheduled,
}

impl From<Message> for VideoChatScheduledMessage {
    fn from(remote: Message) -> Self {
        let Message {
            video_chat_scheduled,
            ..
        } = remote;

        Self {
            video_chat_scheduled: video_chat_scheduled.unwrap(),
        }
    }
}
