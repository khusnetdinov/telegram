use crate::enums::revenue_withdrawal_state::RevenueWithdrawalState;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::transaction_partner_fragment::TransactionPartnerFragment as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct TransactionPartnerFragment {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdrawal_state: Option<RevenueWithdrawalState>,
}
