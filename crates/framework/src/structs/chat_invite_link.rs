use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_invite_link::ChatInviteLink as Remote;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatInviteLink {
    pub invite_link: String,
    pub creator: User,
    pub creates_join_request: bool,
    pub is_primary: bool,
    pub is_revoked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_join_request_count: Option<i64>,
}
impl From<Remote> for ChatInviteLink {
    fn from(remote: Remote) -> Self {
        Self {
            invite_link: remote.invite_link,
            // TODO: #[remote(into)]
            creator: remote.creator.into(),
            creates_join_request: remote.creates_join_request,
            is_primary: remote.is_primary,
            is_revoked: remote.is_revoked,
            name: remote.name,
            expire_date: remote.expire_date,
            member_limit: remote.member_limit,
            pending_join_request_count: remote.pending_join_request_count,
        }
    }
}
