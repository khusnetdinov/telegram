use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::birthdate::Birthdate as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct Birthdate {
    pub day: u8,
    pub month: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u8>,
}
