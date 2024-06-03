use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::video_chat_scheduled::VideoChatScheduled;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoChatScheduledMessage {
    pub video_chat_scheduled: VideoChatScheduled,
}

impl From<Inner> for VideoChatScheduledMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            video_chat_scheduled,
            ..
        } = inner;

        Self {
            video_chat_scheduled: video_chat_scheduled.unwrap(),
        }
    }
}
