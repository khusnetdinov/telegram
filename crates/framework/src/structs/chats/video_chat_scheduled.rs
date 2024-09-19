use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::video_chat_scheduled::VideoChatScheduled as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct VideoChatScheduled {
    pub start_date: i64,
}
