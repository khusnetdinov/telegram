use crate::structs::payments::transaction_partner_fragment::TransactionPartnerFragment;
use crate::structs::payments::transaction_partner_other::TransactionPartnerOther;
use crate::structs::payments::transaction_partner_user::TransactionPartnerUser;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::transaction_partner::TransactionPartner as Remote;
use telegram_macros::{FromRemoteEnum, IntoRemoteEnum};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum, IntoRemoteEnum)]
pub enum TransactionPartner {
    Fragment(TransactionPartnerFragment),
    User(TransactionPartnerUser),
    Other(TransactionPartnerOther),
}
