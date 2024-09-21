use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;
use telegram_bots_api::api::structs::video_chat_scheduled::VideoChatScheduled as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct VideoChatScheduled {
    pub start_date: i64,
}

impl From<IncomingMessage> for VideoChatScheduled {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            video_chat_scheduled,
            ..
        } = remote;

        Self::from(video_chat_scheduled.unwrap())
    }
}
