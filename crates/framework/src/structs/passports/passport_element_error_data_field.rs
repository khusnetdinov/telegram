use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::passport_element_error_data_field::PassportElementErrorDataField as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorDataField {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub source: String,
    pub message: String,
    pub data_hash: String,
    pub field_name: String,
}

impl From<Remote> for PassportElementErrorDataField {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            source: remote.source,
            field_name: remote.field_name,
            data_hash: remote.data_hash,
            message: remote.message,
        }
    }
}

impl From<PassportElementErrorDataField> for Remote {
    fn from(value: PassportElementErrorDataField) -> Self {
        Self {
            kind: value.kind,
            source: value.source,
            field_name: value.field_name,
            data_hash: value.data_hash,
            message: value.message,
        }
    }
}
