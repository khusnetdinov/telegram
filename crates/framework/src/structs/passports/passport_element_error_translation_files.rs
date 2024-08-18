use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::passport_element_error_translation_files::PassportElementErrorTranslationFiles as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorTranslationFiles {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub source: String,
    pub message: String,
    pub file_hashes: Vec<String>,
}

impl From<Remote> for PassportElementErrorTranslationFiles {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            source: remote.source,
            message: remote.message,
            file_hashes: remote.file_hashes,
        }
    }
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
