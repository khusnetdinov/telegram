use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_boost_source_premium::ChatBoostSourcePremium as Remote;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostSourcePremium {
    pub source: String,
    pub user: User,
}
impl From<Remote> for ChatBoostSourcePremium {
    fn from(remote: Remote) -> Self {
        Self {
            source: remote.source,
            user: remote.user.into(),
        }
    }
}
