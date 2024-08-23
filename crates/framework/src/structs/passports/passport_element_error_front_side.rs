use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::passport_element_error_front_side::PassportElementErrorFrontSide as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct PassportElementErrorFrontSide {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub source: String,
    pub message: String,
    pub file_hash: String,
}

impl From<PassportElementErrorFrontSide> for Remote {
    fn from(value: PassportElementErrorFrontSide) -> Self {
        Self {
            kind: value.kind,
            source: value.source,
            message: value.message,
            file_hash: value.file_hash,
        }
    }
}
