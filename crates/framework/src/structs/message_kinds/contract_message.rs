use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::contact::Contact;
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContactMessage {
    pub contact: Contact,
}

impl From<Inner> for ContactMessage {
    fn from(inner: Inner) -> Self {
        let Inner { contact, .. } = inner;

        Self {
            contact: contact.unwrap(),
        }
    }
}
