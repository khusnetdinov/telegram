use crate::enums::background_type::BackgroundType;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_background::ChatBackground as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct ChatBackground {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: BackgroundType,
}
