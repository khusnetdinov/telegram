use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_boost_added::ChatBoostAdded as Remote;
use telegram_bots_api::api::structs::message::Message;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct ChatBoostAdded {
    pub boost_count: u64,
}

impl From<Message> for ChatBoostAdded {
    fn from(remote: Message) -> Self {
        let Message {
            boost_added: Some(boost_added),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(*boost_added)
    }
}
