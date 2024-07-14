use crate::structs::shipping_address::ShippingAddress;
use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::shipping_query::ShippingQuery as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}
impl From<Remote> for ShippingQuery {
    fn from(remote: Remote) -> Self {
        Self {
            id: remote.id,
            // TODO: #[remote(into)]
            from: remote.from.into(),
            invoice_payload: remote.invoice_payload,
            // TODO: #[remote(into)]
            shipping_address: remote.shipping_address.into(),
        }
    }
}
