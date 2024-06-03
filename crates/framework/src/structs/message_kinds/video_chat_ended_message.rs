use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::video_chat_ended::VideoChatEnded;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoChatEndedMessage {
    pub video_chat_ended: VideoChatEnded,
}

impl From<Inner> for VideoChatEndedMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            video_chat_ended, ..
        } = inner;

        Self {
            video_chat_ended: video_chat_ended.unwrap(),
        }
    }
}
