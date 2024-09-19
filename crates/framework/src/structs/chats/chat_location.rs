use crate::structs::location::Location;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_location::ChatLocation as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct ChatLocation {
    pub location: Location,
    pub address: String,
}
