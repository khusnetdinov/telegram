use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::invoice::Invoice;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InvoiceMessage {
    pub invoice: Invoice,
}

impl From<Message> for InvoiceMessage {
    fn from(remote: Message) -> Self {
        let Message { invoice, .. } = remote;

        Self {
            invoice: invoice.unwrap(),
        }
    }
}
