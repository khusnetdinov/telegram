use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::sticker::Sticker;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StickerMessage {
    pub sticker: Sticker,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
}

impl From<Inner> for StickerMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            sticker,
            media_group_id,
            has_media_spoiler,
            ..
        } = inner;

        Self {
            sticker: sticker.unwrap(),
            media_group_id,
            has_media_spoiler,
        }
    }
}
