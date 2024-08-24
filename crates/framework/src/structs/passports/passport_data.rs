use crate::structs::passports::encrypted_credentials::EncryptedCredentials;
use crate::structs::passports::encrypted_passport_element::EncryptedPassportElement;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::passport_data::PassportData as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}

impl From<Message> for PassportData {
    fn from(remote: Message) -> Self {
        let Message { passport_data, .. } = remote;

        Self::from(passport_data.unwrap())
    }
}
