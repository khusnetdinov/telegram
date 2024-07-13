use crate::structs::passport_file::PassportFile;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::encrypted_passport_element::EncryptedPassportElement as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EncryptedPassportElement {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<PassportFile>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_side: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_side: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
}

impl From<Remote> for EncryptedPassportElement {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            hash: remote.hash,
            data: remote.data,
            phone_number: remote.phone_number,
            email: remote.email,
            // TODO: #[remote(option, map, into)]
            files: remote
                .files
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            // TODO: #[remote(option, into)]
            front_side: remote.front_side.map(|inner| inner.to_owned().into()),
            // TODO: #[remote(option, into)]
            reverse_side: remote.reverse_side.map(|inner| inner.to_owned().into()),
            // TODO: #[remote(option, into)]
            selfie: remote.selfie.map(|inner| inner.to_owned().into()),
            // TODO: #[remote(option, map, into)]
            translation: remote
                .translation
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
        }
    }
}
