use crate::structs::chat::Chat;
use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::poll_answer::PollAnswer as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct PollAnswer {
    pub poll_id: String,
    pub option_ids: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voter_chat: Option<Chat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}
