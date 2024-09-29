use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_boost_added::ChatBoostAdded as Remote;
use telegram_bots_api::api::structs::message::Message as IncomingMessage;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct ChatBoostAdded {
    pub boost_count: u64,
}

impl From<IncomingMessage> for ChatBoostAdded {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage { boost_added, .. } = remote;

        Self::from(*boost_added.unwrap())
    }
}
