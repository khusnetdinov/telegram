use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_boost_source_gift_code::ChatBoostSourceGiftCode as Remote;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostSourceGiftCode {
    pub source: String,
    pub user: User,
}
impl From<Remote> for ChatBoostSourceGiftCode {
    fn from(remote: Remote) -> Self {
        Self {
            source: remote.source,
            // TODO: #[remote(into)]
            user: remote.user.into(),
        }
    }
}
