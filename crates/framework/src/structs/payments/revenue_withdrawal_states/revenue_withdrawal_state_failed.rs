use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::revenue_withdrawal_state_failed::RevenueWithdrawalStateFailed as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct RevenueWithdrawalStateFailed {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
