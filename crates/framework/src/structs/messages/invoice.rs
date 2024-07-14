use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::invoice::Invoice as Remote;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InvoiceMessage {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: i64,
}
impl From<Remote> for InvoiceMessage {
    fn from(remote: Remote) -> Self {
        Self {
            title: remote.title,
            description: remote.description,
            start_parameter: remote.start_parameter,
            currency: remote.currency,
            total_amount: remote.total_amount,
        }
    }
}
impl From<Message> for InvoiceMessage {
    fn from(remote: Message) -> Self {
        let Message {
            invoice: Some(invoice),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(invoice)
    }
}
