use crate::structs::passports::encrypted_credentials::EncryptedCredentials;
use crate::structs::passports::encrypted_passport_element::EncryptedPassportElement;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::passport_data::PassportData as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}
impl From<Remote> for PassportData {
    fn from(remote: Remote) -> Self {
        Self {
            // TODO: #[remote(map, into)]
            data: remote
                .data
                .iter()
                .map(|inner| inner.to_owned().into())
                .collect(),
            // TODO: #[remote(into)]
            credentials: remote.credentials.into(),
        }
    }
}
impl From<Message> for PassportData {
    fn from(remote: Message) -> Self {
        let Message {
            passport_data: Some(passport_data),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(passport_data)
    }
}
