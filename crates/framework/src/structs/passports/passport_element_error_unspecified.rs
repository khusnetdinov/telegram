use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::passport_element_error_unspecified::PassportElementErrorUnspecified as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct PassportElementErrorUnspecified {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub source: String,
    pub message: String,
    pub element_hash: String,
}

impl From<PassportElementErrorUnspecified> for Remote {
    fn from(value: PassportElementErrorUnspecified) -> Self {
        Self {
            kind: value.kind,
            source: value.source,
            message: value.message,
            element_hash: value.element_hash,
        }
    }
}
