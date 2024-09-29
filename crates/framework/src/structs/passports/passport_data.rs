use crate::structs::passports::encrypted::encrypted_credentials::EncryptedCredentials;
use crate::structs::passports::encrypted::encrypted_passport_element::EncryptedPassportElement;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;
use telegram_bots_api::api::structs::passport_data::PassportData as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}

impl From<IncomingMessage> for PassportData {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage { passport_data, .. } = remote;

        Self::from(passport_data.unwrap())
    }
}
