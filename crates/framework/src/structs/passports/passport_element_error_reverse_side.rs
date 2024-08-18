use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::passport_element_error_reverse_side::PassportElementErrorReverseSide as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorReverseSide {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub source: String,
    pub file_hash: String,
    pub message: String,
}

impl From<Remote> for PassportElementErrorReverseSide {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            source: remote.source,
            message: remote.message,
            file_hash: remote.file_hash,
        }
    }
}

impl From<PassportElementErrorReverseSide> for Remote {
    fn from(value: PassportElementErrorReverseSide) -> Self {
        Self {
            kind: value.kind,
            source: value.source,
            message: value.message,
            file_hash: value.file_hash,
        }
    }
}
