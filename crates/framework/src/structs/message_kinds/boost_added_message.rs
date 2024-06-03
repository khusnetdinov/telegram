use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_boost_added::ChatBoostAdded;
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostAddedMessage {
    pub boost_added: Box<ChatBoostAdded>,
}

impl From<Inner> for ChatBoostAddedMessage {
    fn from(inner: Inner) -> Self {
        let Inner { boost_added, .. } = inner;

        Self {
            boost_added: boost_added.unwrap(),
        }
    }
}
