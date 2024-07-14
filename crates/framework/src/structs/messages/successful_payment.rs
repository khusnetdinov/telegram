use crate::structs::order_info::OrderInfo;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::successful_payment::SuccessfulPayment as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
}
impl From<Remote> for SuccessfulPayment {
    fn from(remote: Remote) -> Self {
        Self {
            currency: remote.currency,
            total_amount: remote.total_amount,
            invoice_payload: remote.invoice_payload,
            telegram_payment_charge_id: remote.telegram_payment_charge_id,
            provider_payment_charge_id: remote.provider_payment_charge_id,
            shipping_option_id: remote.shipping_option_id,
            // TODO: #[remote(option, into)]
            order_info: remote.order_info.map(|inner| inner.into()),
        }
    }
}

impl From<Message> for SuccessfulPayment {
    fn from(remote: Message) -> Self {
        let Message {
            successful_payment: Some(successful_payment),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(successful_payment)
    }
}
