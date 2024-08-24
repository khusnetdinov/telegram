use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::encrypted_credentials::EncryptedCredentials as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct EncryptedCredentials {
    pub data: String,
    pub hash: String,
    pub secret: String,
}
