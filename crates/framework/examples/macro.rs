use crate::structs::encrypted_credentials::EncryptedCredentials;
use crate::structs::encrypted_passport_element::EncryptedPassportElement;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::passport_data::PassportData as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}

impl From<Remote> for PassportData {
    fn from(remote: Remote) -> Self {
        Self {
            data: remote.data,
            credentials: remote.credentials,
        }
    }
}
