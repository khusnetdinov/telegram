use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::video_chat_scheduled::VideoChatScheduled as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct VideoChatScheduled {
    pub start_date: i64,
}

impl From<Message> for VideoChatScheduled {
    fn from(remote: Message) -> Self {
        let Message {
            video_chat_scheduled,
            ..
        } = remote;

        Self::from(video_chat_scheduled.unwrap())
    }
}
