use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::passport_data::PassportData;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportDataMessage {
    pub passport_data: PassportData,
}

impl From<Message> for PassportDataMessage {
    fn from(remote: Message) -> Self {
        let Message { passport_data, .. } = remote;

        Self {
            passport_data: passport_data.unwrap(),
        }
    }
}
