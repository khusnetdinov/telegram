use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::video_chat_ended::VideoChatEnded as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct VideoChatEnded {
    pub duration: i64,
}
