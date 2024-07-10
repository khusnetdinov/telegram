use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::shipping_address::ShippingAddress as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}
impl From<Remote> for ShippingAddress {
    fn from(remote: Remote) -> Self {
        Self {
            country_code: remote.country_code,
            state: remote.state,
            city: remote.city,
            street_line1: remote.street_line1,
            street_line2: remote.street_line2,
            post_code: remote.post_code,
        }
    }
}
