use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::encrypted_credentials::EncryptedCredentials as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EncryptedCredentials {
    pub data: String,
    pub hash: String,
    pub secret: String,
}
impl From<Remote> for EncryptedCredentials {
    fn from(remote: Remote) -> Self {
        Self {
            data: remote.data,
            hash: remote.hash,
            secret: remote.secret,
        }
    }
}
