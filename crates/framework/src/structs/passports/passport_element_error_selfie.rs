use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::passport_element_error_selfie::PassportElementErrorSelfie as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorSelfie {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub file_hash: String,
    pub source: String,
    pub message: String,
}

impl From<Remote> for PassportElementErrorSelfie {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            source: remote.source,
            message: remote.message,
            file_hash: remote.file_hash,
        }
    }
}

impl From<PassportElementErrorSelfie> for Remote {
    fn from(value: PassportElementErrorSelfie) -> Self {
        Self {
            kind: value.kind,
            source: value.source,
            message: value.message,
            file_hash: value.file_hash,
        }
    }
}
