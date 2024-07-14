use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::background_type_wallpaper::BackgroundTypeWallpaper as Remote;
use telegram_bots_api::api::structs::document::Document;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
impl From<Remote> for BackgroundTypeWallpaper {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            document: remote.document,
            dark_theme_dimming: remote.dark_theme_dimming,
            is_blurred: remote.is_blurred,
            is_moving: remote.is_moving,
        }
    }
}
