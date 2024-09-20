use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::invoice::Invoice as Remote;
use telegram_bots_api::api::structs::message::Message;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct InvoiceMessage {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: i64,
}

impl From<Message> for InvoiceMessage {
    fn from(remote: Message) -> Self {
        let Message { invoice, .. } = remote;

        Self::from(invoice.unwrap())
    }
}
