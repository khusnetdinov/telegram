use crate::structs::media::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::paid_media_photo::PaidMediaPhoto as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct PaidMediaPhoto {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub photo: Vec<PhotoSize>,
}
