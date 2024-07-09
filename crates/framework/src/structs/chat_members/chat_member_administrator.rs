use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_member_administrator::ChatMemberAdministrator as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberAdministrator {
    pub status: String,
    pub user: User,
    pub can_be_edited: bool,
    pub is_anonymous: bool,
    pub can_manage_chat: bool,
    pub can_delete_messages: bool,
    pub can_manage_video_chats: bool,
    pub can_restrict_members: bool,
    pub can_promote_members: bool,
    pub can_change_info: bool,
    pub can_invite_users: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
}
impl From<Remote> for ChatMemberAdministrator {
    fn from(remote: Remote) -> Self {
        Self {
            status: remote.status,
            // TODO: #[remote(into)]
            user: remote.user.into(),
            can_be_edited: remote.can_be_edited,
            is_anonymous: remote.is_anonymous,
            can_manage_chat: remote.can_manage_chat,
            can_delete_messages: remote.can_delete_messages,
            can_manage_video_chats: remote.can_manage_video_chats,
            can_restrict_members: remote.can_restrict_members,
            can_promote_members: remote.can_promote_members,
            can_change_info: remote.can_change_info,
            can_invite_users: remote.can_invite_users,
            can_post_messages: remote.can_post_messages,
            can_edit_messages: remote.can_edit_messages,
            can_pin_messages: remote.can_pin_messages,
            can_post_stories: remote.can_post_stories,
            can_edit_stories: remote.can_edit_stories,
            can_delete_stories: remote.can_delete_stories,
            can_manage_topics: remote.can_manage_topics,
            custom_title: remote.custom_title,
        }
    }
}
