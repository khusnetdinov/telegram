use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::video_chat_scheduled::VideoChatScheduled as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoChatScheduled {
    pub start_date: i64,
}
impl From<Remote> for VideoChatScheduled {
    fn from(remote: Remote) -> Self {
        Self {
            start_date: remote.start_date,
        }
    }
}
impl From<Message> for VideoChatScheduled {
    fn from(remote: Message) -> Self {
        let Message {
            video_chat_scheduled: Some(video_chat_scheduled),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(video_chat_scheduled)
    }
}
