use crate::structs::media::video::Video;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::paid_media_video::PaidMediaVideo as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct PaidMediaVideo {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub video: Video,
}
