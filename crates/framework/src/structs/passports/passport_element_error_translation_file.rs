use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::passport_element_error_translation_file::PassportElementErrorTranslationFile as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct PassportElementErrorTranslationFile {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub file_hash: String,
    pub source: String,
    pub message: String,
}

impl From<PassportElementErrorTranslationFile> for Remote {
    fn from(value: PassportElementErrorTranslationFile) -> Self {
        Self {
            kind: value.kind,
            source: value.source,
            message: value.message,
            file_hash: value.file_hash,
        }
    }
}
