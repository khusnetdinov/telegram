use crate::structs::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::shared_user::SharedUser as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SharedUser {
    pub user_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
}
impl From<Remote> for SharedUser {
    fn from(remote: Remote) -> Self {
        Self {
            user_id: remote.user_id,
            first_name: remote.first_name,
            last_name: remote.last_name,
            username: remote.username,
            // TODO: #[remote(option, map, into)]
            photo: remote
                .photo
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
        }
    }
}
