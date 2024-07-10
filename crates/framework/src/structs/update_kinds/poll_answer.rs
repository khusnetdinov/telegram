use crate::structs::chat::Chat;
use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::poll_answer::PollAnswer as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PollAnswer {
    pub poll_id: String,
    pub option_ids: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voter_chat: Option<Chat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}
impl From<Remote> for PollAnswer {
    fn from(remote: Remote) -> Self {
        Self {
            poll_id: remote.poll_id,
            option_ids: remote.option_ids,
            // TODO: #[remote(option)]
            voter_chat: remote.voter_chat.map(|inner| inner.into()),
            // TODO: #[remote(option)]
            user: remote.user.map(|inner| inner.into()),
        }
    }
}
