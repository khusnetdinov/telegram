use crate::structs::shipping_address::ShippingAddress;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::order_info::OrderInfo as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct OrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>,
}
