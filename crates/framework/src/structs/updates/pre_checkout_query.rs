use crate::structs::order_info::OrderInfo;
use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::pre_checkout_query::PreCheckoutQuery as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
}
impl From<Remote> for PreCheckoutQuery {
    fn from(remote: Remote) -> Self {
        Self {
            id: remote.id,
            // TODO: #[remote(into)]
            from: remote.from.into(),
            currency: remote.currency,
            total_amount: remote.total_amount,
            invoice_payload: remote.invoice_payload,
            shipping_option_id: remote.shipping_option_id,
            // TODO: #[remote(option, into)]
            order_info: remote.order_info.map(|inner| inner.into()),
        }
    }
}
