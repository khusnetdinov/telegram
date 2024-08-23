use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::file::File as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct File {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: i64,
    pub file_path: String,
}
