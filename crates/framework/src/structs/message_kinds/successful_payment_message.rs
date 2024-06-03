use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::successful_payment::SuccessfulPayment;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SuccessfulPaymentMessage {
    pub successful_payment: SuccessfulPayment,
}

impl From<Inner> for SuccessfulPaymentMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            successful_payment, ..
        } = inner;

        Self {
            successful_payment: successful_payment.unwrap(),
        }
    }
}
