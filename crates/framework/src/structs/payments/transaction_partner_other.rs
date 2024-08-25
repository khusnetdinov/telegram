use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::transaction_partner_other::TransactionPartnerOther as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct TransactionPartnerOther {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
