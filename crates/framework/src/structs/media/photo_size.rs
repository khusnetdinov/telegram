use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::photo_size::PhotoSize as Remote;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    pub file_size: Option<i64>,
}
impl From<Remote> for PhotoSize {
    fn from(remote: Remote) -> Self {
        Self {
            file_id: remote.file_id,
            file_unique_id: remote.file_unique_id,
            width: remote.width,
            height: remote.height,
            file_size: remote.file_size,
        }
    }
}
