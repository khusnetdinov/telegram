use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::transaction_partner_telegram_ads::TransactionPartnerTelegramAds as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct TransactionPartnerTelegramAds {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
