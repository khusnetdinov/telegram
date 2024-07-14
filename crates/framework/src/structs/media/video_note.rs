use crate::structs::media::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::video_note::VideoNote as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: i64,
    pub duration: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
impl From<Remote> for VideoNote {
    fn from(remote: Remote) -> Self {
        Self {
            file_id: remote.file_id,
            file_unique_id: remote.file_unique_id,
            length: remote.length,
            duration: remote.duration,
            // TODO: #[remote(option, into)]
            thumbnail: remote.thumbnail.map(|inner| inner.into()),
            file_size: remote.file_size,
        }
    }
}
