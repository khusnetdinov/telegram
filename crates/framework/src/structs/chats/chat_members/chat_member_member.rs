use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_member_member::ChatMemberMember as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct ChatMemberMember {
    pub status: String,
    pub user: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}
