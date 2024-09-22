use crate::structs::geo::location::Location;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::business_location::BusinessLocation as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct BusinessLocation {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}
