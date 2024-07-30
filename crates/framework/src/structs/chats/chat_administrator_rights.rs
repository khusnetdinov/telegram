use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_administrator_rights::ChatAdministratorRights as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatAdministratorRights {
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
}
impl From<Remote> for ChatAdministratorRights {
    fn from(remote: Remote) -> Self {
        Self {
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
        }
    }
}

impl From<ChatAdministratorRights> for Remote {
    fn from(value: ChatAdministratorRights) -> Self {
        Self {
            is_anonymous: value.is_anonymous,
            can_manage_chat: value.can_manage_chat,
            can_delete_messages: value.can_delete_messages,
            can_manage_video_chats: value.can_manage_video_chats,
            can_restrict_members: value.can_restrict_members,
            can_promote_members: value.can_promote_members,
            can_change_info: value.can_change_info,
            can_invite_users: value.can_invite_users,
            can_post_messages: value.can_post_messages,
            can_edit_messages: value.can_edit_messages,
            can_pin_messages: value.can_pin_messages,
            can_post_stories: value.can_post_stories,
            can_edit_stories: value.can_edit_stories,
            can_delete_stories: value.can_delete_stories,
            can_manage_topics: value.can_manage_topics,
        }
    }
}
