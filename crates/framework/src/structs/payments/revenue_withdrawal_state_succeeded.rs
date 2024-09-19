use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::revenue_withdrawal_state_succeeded::RevenueWithdrawalStateSucceeded as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct RevenueWithdrawalStateSucceeded {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub date: i64,
    pub url: String,
}
