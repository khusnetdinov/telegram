use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::invoice::Invoice;
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InvoiceMessage {
    pub invoice: Invoice,
}

impl From<Inner> for InvoiceMessage {
    fn from(inner: Inner) -> Self {
        let Inner { invoice, .. } = inner;

        Self {
            invoice: invoice.unwrap(),
        }
    }
}
