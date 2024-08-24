use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::passport_element_error_data_field::PassportElementErrorDataField as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct PassportElementErrorDataField {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub source: String,
    pub message: String,
    pub data_hash: String,
    pub field_name: String,
}
