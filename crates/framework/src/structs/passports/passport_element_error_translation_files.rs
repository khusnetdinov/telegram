use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::passport_element_error_translation_files::PassportElementErrorTranslationFiles as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct PassportElementErrorTranslationFiles {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub source: String,
    pub message: String,
    pub file_hashes: Vec<String>,
}

impl From<PassportElementErrorTranslationFiles> for Remote {
    fn from(value: PassportElementErrorTranslationFiles) -> Self {
        Self {
            kind: value.kind,
            source: value.source,
            message: value.message,
            file_hashes: value.file_hashes,
        }
    }
}
