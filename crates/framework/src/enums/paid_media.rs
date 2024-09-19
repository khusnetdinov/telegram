use crate::structs::payments::paid_media_photo::PaidMediaPhoto;
use crate::structs::payments::paid_media_preview::PaidMediaPreview;
use crate::structs::payments::paid_media_video::PaidMediaVideo;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::paid_media::PaidMedia as Remote;
use telegram_macros::{FromRemoteEnum, IntoRemoteEnum};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum, IntoRemoteEnum)]
pub enum PaidMedia {
    Preview(PaidMediaPreview),
    Photo(PaidMediaPhoto),
    Video(PaidMediaVideo),
}
