use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_member_left::ChatMemberLeft as Remote;
use telegram_bots_api::api::structs::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberLeft {
    pub status: String,
    pub user: User,
}
impl From<Remote> for ChatMemberLeft {
    fn from(remote: Remote) -> Self {
        Self {
            status: remote.status,
            user: remote.user,
        }
    }
}
