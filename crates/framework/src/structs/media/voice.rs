use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::voice::Voice as Remote;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Voice {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
impl From<Remote> for Voice {
    fn from(remote: Remote) -> Self {
        Self {
            file_id: remote.file_id,
            file_unique_id: remote.file_unique_id,
            duration: remote.duration,
            mime_type: remote.mime_type,
            file_size: remote.file_size,
        }
    }
}
