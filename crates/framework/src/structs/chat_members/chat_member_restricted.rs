use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_member_restricted::ChatMemberRestricted as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberRestricted {
    pub status: String,
    pub user: User,
    pub is_member: bool,
    pub can_send_messages: bool,
    pub can_send_audios: bool,
    pub can_send_documents: bool,
    pub can_send_photos: bool,
    pub can_send_videos: bool,
    pub can_send_video_notes: bool,
    pub can_send_voice_notes: bool,
    pub can_send_polls: bool,
    pub can_send_other_messages: bool,
    pub can_add_web_page_previews: bool,
    pub can_change_info: bool,
    pub can_invite_users: bool,
    pub can_pin_messages: bool,
    pub can_manage_topics: bool,
    pub until_date: i64,
}
impl From<Remote> for ChatMemberRestricted {
    fn from(remote: Remote) -> Self {
        Self {
            status: remote.status,
            // TODO: #[remote(into)]
            user: remote.user.into(),
            is_member: remote.is_member,
            can_send_messages: remote.can_send_messages,
            can_send_audios: remote.can_send_audios,
            can_send_documents: remote.can_send_documents,
            can_send_photos: remote.can_send_photos,
            can_send_videos: remote.can_send_videos,
            can_send_video_notes: remote.can_send_video_notes,
            can_send_voice_notes: remote.can_send_voice_notes,
            can_send_polls: remote.can_send_polls,
            can_send_other_messages: remote.can_send_other_messages,
            can_add_web_page_previews: remote.can_add_web_page_previews,
            can_change_info: remote.can_change_info,
            can_invite_users: remote.can_invite_users,
            can_pin_messages: remote.can_pin_messages,
            can_manage_topics: remote.can_manage_topics,
            until_date: remote.until_date,
        }
    }
}
