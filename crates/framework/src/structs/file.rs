use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::file::File as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct File {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: i64,
    pub file_path: String,
}

impl From<Remote> for File {
    fn from(remote: Remote) -> Self {
        Self {
            file_id: remote.file_id,
            file_unique_id: remote.file_unique_id,
            file_size: remote.file_size,
            file_path: remote.file_path,
        }
    }
}
