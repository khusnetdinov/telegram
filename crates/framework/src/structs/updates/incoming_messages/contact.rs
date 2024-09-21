use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::contact::Contact as Remote;
use telegram_bots_api::api::structs::message::Message as IncomingMessage;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<i64>,
    pub vcard: Option<String>,
}

impl From<IncomingMessage> for Contact {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage { contact, .. } = remote;

        Self::from(contact.unwrap())
    }
}
