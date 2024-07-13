use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::video_chat_ended::VideoChatEnded as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoChatEnded {
    pub duration: i64,
}
impl From<Remote> for VideoChatEnded {
    fn from(remote: Remote) -> Self {
        Self {
            duration: remote.duration,
        }
    }
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
