use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::successful_payment::SuccessfulPayment;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SuccessfulPaymentMessage {
    pub successful_payment: SuccessfulPayment,
}

impl From<Message> for SuccessfulPaymentMessage {
    fn from(remote: Message) -> Self {
        let Message {
            successful_payment, ..
        } = remote;

        Self {
            successful_payment: successful_payment.unwrap(),
        }
    }
}
