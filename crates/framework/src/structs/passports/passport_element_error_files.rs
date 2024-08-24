use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::passport_element_error_files::PassportElementErrorFiles as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct PassportElementErrorFiles {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub source: String,
    pub message: String,
    pub file_hashes: Vec<String>,
}
