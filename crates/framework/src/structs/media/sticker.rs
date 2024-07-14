use crate::structs::media::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::file::File;
use telegram_bots_api::api::structs::mask_position::MaskPosition;
use telegram_bots_api::api::structs::sticker::Sticker as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sticker {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub width: i64,
    pub file_id: String,
    pub file_unique_id: String,
    pub height: i64,
    pub is_animated: bool,
    pub is_video: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_animation: Option<File>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
impl From<Remote> for Sticker {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            width: remote.width,
            file_id: remote.file_id,
            file_unique_id: remote.file_unique_id,
            height: remote.height,
            is_animated: remote.is_animated,
            is_video: remote.is_video,
            // TODO: #[remote(option, into)]
            thumbnail: remote.thumbnail.map(|inner| inner.into()),
            emoji: remote.emoji,
            set_name: remote.set_name,
            premium_animation: remote.premium_animation,
            mask_position: remote.mask_position,
            custom_emoji_id: remote.custom_emoji_id,
            needs_repainting: remote.needs_repainting,
            file_size: remote.file_size,
        }
    }
}
