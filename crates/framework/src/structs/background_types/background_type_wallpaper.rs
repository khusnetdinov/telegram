use crate::structs::media::document::Document;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::background_type_wallpaper::BackgroundTypeWallpaper as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct BackgroundTypeWallpaper {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub document: Document,
    pub dark_theme_dimming: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_blurred: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_moving: Option<bool>,
}
