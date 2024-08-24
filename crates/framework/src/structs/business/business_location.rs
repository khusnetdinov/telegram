use crate::structs::location::Location;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::business_location::BusinessLocation as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct BusinessLocation {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}
