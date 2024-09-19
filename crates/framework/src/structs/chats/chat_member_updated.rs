use crate::enums::chat_member::ChatMember;
use crate::structs::chat::Chat;
use crate::structs::chats::chat_invite_link::ChatInviteLink;
use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_member_updated::ChatMemberUpdated as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
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
