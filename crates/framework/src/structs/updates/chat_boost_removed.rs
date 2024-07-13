use crate::enums::chat_boost_source::ChatBoostSource;
use crate::structs::chat::Chat;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_boost_removed::ChatBoostRemoved as Remote;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostRemoved {
    pub chat: Chat,
    pub boost_id: String,
    pub remove_date: i64,
    pub source: ChatBoostSource,
}

impl From<Remote> for ChatBoostRemoved {
    fn from(remote: Remote) -> Self {
        Self {
            // TODO: #[remote(into)]
            chat: remote.chat.into(),
            boost_id: remote.boost_id,
            remove_date: remote.remove_date,
            // TODO: #[remote(into)]
            source: remote.source.into(),
        }
    }
}
