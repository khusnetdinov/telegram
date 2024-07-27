use crate::structs::location::Location;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::business_location::BusinessLocation as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BusinessLocation {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

impl From<Remote> for BusinessLocation {
    fn from(remote: Remote) -> Self {
        Self {
            address: remote.address,
            // TODO: #[remote(option, into)]
            location: remote.location.map(|inner| inner.into()),
        }
    }
}
