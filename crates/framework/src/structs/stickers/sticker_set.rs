use crate::structs::media::photo_size::PhotoSize;
use crate::structs::sticker::Sticker;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::sticker_set::StickerSet as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct StickerSet {
    pub name: String,
    pub title: String,
    pub sticker_type: String,
    pub stickers: Vec<Sticker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
}
