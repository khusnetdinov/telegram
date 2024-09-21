use crate::enums::transaction_partner::TransactionPartner;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::star_transaction::StarTransaction as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct StarTransaction {
    pub id: i64,
    pub amount: u64,
    pub date: i64,
    pub source: Option<TransactionPartner>,
    pub receiver: Option<TransactionPartner>,
}
