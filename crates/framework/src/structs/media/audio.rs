use crate::structs::media::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::audio::Audio as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Audio {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
}
impl From<Remote> for Audio {
    fn from(remote: Remote) -> Self {
        Self {
            file_id: remote.file_id,
            file_unique_id: remote.file_unique_id,
            duration: remote.duration,
            performer: remote.performer,
            title: remote.title,
            file_name: remote.file_name,
            mime_type: remote.mime_type,
            file_size: remote.file_size,
            // TODO: #[remote(option, into)]
            thumbnail: remote.thumbnail.map(|inner| inner.into()),
        }
    }
}
