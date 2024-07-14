use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::business_connection::BusinessConnection as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BusinessConnection {
    pub id: String,
    pub user: User,
    pub user_chat_id: u64,
    pub date: u64,
    pub can_reply: bool,
    pub is_enabled: bool,
}

impl From<Remote> for BusinessConnection {
    fn from(remote: Remote) -> Self {
        Self {
            id: remote.id,
            // TODO: #[remote(into)]
            user: remote.user.into(),
            user_chat_id: remote.user_chat_id,
            date: remote.date,
            can_reply: remote.can_reply,
            is_enabled: remote.is_enabled,
        }
    }
}
