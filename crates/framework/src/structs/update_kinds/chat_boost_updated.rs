use crate::structs::chat::Chat;
use crate::structs::chat_boost::ChatBoost;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_boost_updated::ChatBoostUpdated as Remote;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostUpdated {
    pub chat: Chat,
    pub boost: ChatBoost,
}

impl From<Remote> for ChatBoostUpdated {
    fn from(remote: Remote) -> Self {
        Self {
            // TODO: #[remote(into)]
            chat: remote.chat.into(),
            // TODO: #[remote(into)]
            boost: remote.boost.into(),
        }
    }
}
