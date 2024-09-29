use crate::structs::payments::labeled_price::LabeledPrice;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::shipping_option::ShippingOption as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct ShippingOption {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}
