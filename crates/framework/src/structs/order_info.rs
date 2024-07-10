use crate::structs::shipping_address::ShippingAddress;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::order_info::OrderInfo as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>,
}
impl From<Remote> for OrderInfo {
    fn from(remote: Remote) -> Self {
        Self {
            name: remote.name,
            phone_number: remote.phone_number,
            email: remote.email,
            // TODO: #[remote(option)]
            shipping_address: remote.shipping_address.map(|inner| inner.into()),
        }
    }
}
