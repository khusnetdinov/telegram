use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::contact::Contact as Remote;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<i64>,
    pub vcard: Option<String>,
}
impl From<Remote> for Contact {
    fn from(remote: Remote) -> Self {
        Self {
            phone_number: remote.phone_number,
            first_name: remote.first_name,
            last_name: remote.last_name,
            user_id: remote.user_id,
            vcard: remote.vcard,
        }
    }
}

impl From<Message> for Contact {
    fn from(remote: Message) -> Self {
        let Message { contact, .. } = remote;

        Self::from(contact.unwrap())
    }
}
