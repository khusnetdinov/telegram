use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_member_member::ChatMemberMember as Remote;
use telegram_bots_api::api::structs::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberMember {
    pub status: String,
    pub user: User,
}
impl From<Remote> for ChatMemberMember {
    fn from(remote: Remote) -> Self {
        Self {
            status: remote.status,
            user: remote.user,
        }
    }
}
