use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::passport_element_error_unspecified::PassportElementErrorUnspecified as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct PassportElementErrorUnspecified {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub source: String,
    pub message: String,
    pub element_hash: String,
}
