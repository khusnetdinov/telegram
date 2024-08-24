use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::contact::Contact as Remote;
use telegram_bots_api::api::structs::message::Message;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<i64>,
    pub vcard: Option<String>,
}

impl From<Message> for Contact {
    fn from(remote: Message) -> Self {
        let Message { contact, .. } = remote;

        Self::from(contact.unwrap())
    }
}
