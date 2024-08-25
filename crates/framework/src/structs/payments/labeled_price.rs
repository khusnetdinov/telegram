use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::labeled_price::LabeledPrice as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: i64,
}
