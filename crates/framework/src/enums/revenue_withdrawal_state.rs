use crate::structs::payments::revenue_withdrawal_state_failed::RevenueWithdrawalStateFailed;
use crate::structs::payments::revenue_withdrawal_state_pending::RevenueWithdrawalStatePending;
use crate::structs::payments::revenue_withdrawal_state_succeeded::RevenueWithdrawalStateSucceeded;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::revenue_withdrawal_state::RevenueWithdrawalState as Remote;
use telegram_macros::{FromRemoteEnum, IntoRemoteEnum};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum, IntoRemoteEnum)]
pub enum RevenueWithdrawalState {
    Pending(RevenueWithdrawalStatePending),
    Succeeded(RevenueWithdrawalStateSucceeded),
    Failed(RevenueWithdrawalStateFailed),
}
