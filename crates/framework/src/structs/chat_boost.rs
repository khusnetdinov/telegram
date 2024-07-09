use crate::enums::chat_boost_source::ChatBoostSource;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_boost::ChatBoost as Remote;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoost {
    pub boost_id: String,
    pub add_date: i64,
    pub expiration_date: i64,
    pub source: ChatBoostSource,
}

impl From<Remote> for ChatBoost {
    fn from(remote: Remote) -> Self {
        Self {
            boost_id: remote.boost_id,
            add_date: remote.add_date,
            expiration_date: remote.expiration_date,
            // TODO: #[remote(into)]
            source: remote.source.into(),
        }
    }
}
