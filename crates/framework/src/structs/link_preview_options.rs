use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::link_preview_options::LinkPreviewOptions as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LinkPreviewOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_small_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_large_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_above_text: Option<bool>,
}
impl From<Remote> for LinkPreviewOptions {
    fn from(remote: Remote) -> Self {
        Self {
            is_disabled: remote.is_disabled,
            url: remote.url,
            prefer_small_media: remote.prefer_small_media,
            prefer_large_media: remote.prefer_large_media,
            show_above_text: remote.show_above_text,
        }
    }
}
