use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_boost_source_giveaway::ChatBoostSourceGiveaway as Remote;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostSourceGiveaway {
    pub source: String,
    pub giveaway_message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unclaimed: Option<bool>,
}

impl From<Remote> for ChatBoostSourceGiveaway {
    fn from(remote: Remote) -> Self {
        Self {
            source: remote.source,
            giveaway_message_id: remote.giveaway_message_id,
            // TODO: #[remote(option)]
            user: remote.user.map(|inner| inner.into()),
            is_unclaimed: remote.is_unclaimed,
        }
    }
}
