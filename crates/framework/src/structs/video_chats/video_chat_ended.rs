use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::video_chat_ended::VideoChatEnded as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct VideoChatEnded {
    pub duration: i64,
}

impl From<Message> for VideoChatEnded {
    fn from(remote: Message) -> Self {
        let Message {
            video_chat_ended: Some(video_chat_ended),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(video_chat_ended)
    }
}
