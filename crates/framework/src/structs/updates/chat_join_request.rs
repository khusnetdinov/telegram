use crate::structs::chat::Chat;
use crate::structs::chat_invite_link::ChatInviteLink;
use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_join_request::ChatJoinRequest as Remote;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatJoinRequest {
    pub chat: Chat,
    pub from: User,
    pub user_chat_id: i64,
    pub date: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
}

impl From<Remote> for ChatJoinRequest {
    fn from(remote: Remote) -> Self {
        Self {
            // TODO: #[remote(into)]
            chat: remote.chat.into(),
            // TODO: #[remote(into)]
            from: remote.from.into(),
            user_chat_id: remote.user_chat_id,
            date: remote.date,
            bio: remote.bio,
            // TODO: #[remote(option)]
            invite_link: remote.invite_link.map(|inner| inner.into()),
        }
    }
}
