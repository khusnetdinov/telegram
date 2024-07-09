use crate::enums::chat_member::ChatMember;
use crate::structs::chat::Chat;
use crate::structs::chat_invite_link::ChatInviteLink;
use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_member_updated::ChatMemberUpdated as Remote;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberUpdated {
    pub chat: Chat,
    pub from: User,
    pub date: i64,
    pub old_chat_member: ChatMember,
    pub new_chat_member: ChatMember,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_join_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_chat_folder_invite_link: Option<bool>,
}
impl From<Remote> for ChatMemberUpdated {
    fn from(remote: Remote) -> Self {
        Self {
            // TODO: #[remote(into)]
            chat: remote.chat.into(),
            // TODO: #[remote(into)]
            from: remote.from.into(),
            date: remote.date,
            // TODO: #[remote(into)]
            old_chat_member: remote.old_chat_member.into(),
            // TODO: #[remote(into)]
            new_chat_member: remote.new_chat_member.into(),
            // TODO: #[remote(option)]
            invite_link: remote.invite_link.map(|inner| inner.into()),
            via_join_request: remote.via_join_request,
            via_chat_folder_invite_link: remote.via_chat_folder_invite_link,
        }
    }
}
