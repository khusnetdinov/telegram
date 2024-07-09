use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_member_owner::ChatMemberOwner as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberOwner {
    pub status: String,
    pub user: User,
    pub is_anonymous: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
}
impl From<Remote> for ChatMemberOwner {
    fn from(remote: Remote) -> Self {
        Self {
            status: remote.status,
            // TODO: #[remote(into)]
            user: remote.user.into(),
            is_anonymous: remote.is_anonymous,
            custom_title: remote.custom_title,
        }
    }
}
