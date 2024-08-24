use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_member_owner::ChatMemberOwner as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct ChatMemberOwner {
    pub status: String,
    pub user: User,
    pub is_anonymous: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
}
