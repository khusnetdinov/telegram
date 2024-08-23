use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::passport_element_error_data_field::PassportElementErrorDataField as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct PassportElementErrorDataField {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub source: String,
    pub message: String,
    pub data_hash: String,
    pub field_name: String,
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
