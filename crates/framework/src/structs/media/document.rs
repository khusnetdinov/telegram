use crate::structs::media::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::document::Document as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Document {
    pub file_id: String,
    pub file_unique_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
impl From<Remote> for Document {
    fn from(remote: Remote) -> Self {
        Self {
            file_id: remote.file_id,
            file_unique_id: remote.file_unique_id,
            // TODO: #[remote(option, into)]
            thumbnail: remote.thumbnail.map(|inner| inner.into()),
            file_name: remote.file_name,
            mime_type: remote.mime_type,
            file_size: remote.file_size,
        }
    }
}
