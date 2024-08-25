use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::revenue_withdrawal_state_pending::RevenueWithdrawalStatePending as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct RevenueWithdrawalStatePending {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
