use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::passport_file::PassportFile as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportFile {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: i64,
    pub file_date: i64,
}
impl From<Remote> for PassportFile {
    fn from(remote: Remote) -> Self {
        Self {
            file_id: remote.file_id,
            file_unique_id: remote.file_unique_id,
            file_size: remote.file_size,
            file_date: remote.file_date,
        }
    }
}
