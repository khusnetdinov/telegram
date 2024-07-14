use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_member_banned::ChatMemberBanned as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberBanned {
    pub status: String,
    pub user: User,
    pub until_date: i64,
}
impl From<Remote> for ChatMemberBanned {
    fn from(remote: Remote) -> Self {
        Self {
            status: remote.status,
            // TODO: #[remote(into)]
            user: remote.user.into(),
            until_date: remote.until_date,
        }
    }
}
