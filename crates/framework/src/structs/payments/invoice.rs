use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::invoice::Invoice as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: i64,
}
