use crate::structs::payments::star_transaction::StarTransaction;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::star_transactions::StarTransactions as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct StarTransactions {
    pub transactions: Vec<StarTransaction>,
}
