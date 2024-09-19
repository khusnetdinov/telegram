use crate::enums::reaction_type::ReactionType;
use crate::feature::business::{BusinessIntro, BusinessLocation, BusinessOpeningHours};
use crate::structs::chat::Chat;
use crate::structs::chats::chat_location::ChatLocation;
use crate::structs::chats::chat_permissions::ChatPermissions;
use crate::structs::chats::chat_photo::ChatPhoto;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::birthdate::Birthdate;
use telegram_bots_api::api::structs::chat_full_info::ChatFullInfo as Remote;
use telegram_bots_api::api::structs::message::Message as RemoteMessage;

use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct ChatFullInfo {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: i64,
    pub max_reaction_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_usernames: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<Birthdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_chat: Option<Box<Chat>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_reactions: Option<Vec<ReactionType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_custom_emoji_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_accent_color_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_background_custom_emoji_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_expiration_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_private_forwards: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_restricted_voice_and_video_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_to_send_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_by_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<RemoteMessage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_aggressive_anti_spam_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_hidden_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_visible_history: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ChatLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unrestrict_boost_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_sticker_set_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_intro: Option<BusinessIntro>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_location: Option<BusinessLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_opening_hours: Option<BusinessOpeningHours>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_paid_media: Option<bool>,
}
