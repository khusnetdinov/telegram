use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::video_chat_started::VideoChatStarted as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct VideoChatStarted {}
