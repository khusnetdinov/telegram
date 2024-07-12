use telegram_bots_api::api::structs::contact::Contact as Remote;
use telegram_bots_api::api::structs::message::Message;
use telegram_macros::FromRemote;

#[derive(FromRemote)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}

// impl From<Message> for Contact {
//     fn from(remote: Message) -> Self {
//         let Message { contact, .. } = remote;
//
//         Self {
//             contact: contact.unwrap(),
//         }
//     }
// }
