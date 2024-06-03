use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::video_chat_started::VideoChatStarted;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoChatStartedMessage {
    pub video_chat_started: VideoChatStarted,
}

impl From<Inner> for VideoChatStartedMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            video_chat_started, ..
        } = inner;

        Self {
            video_chat_started: video_chat_started.unwrap(),
        }
    }
}
