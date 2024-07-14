use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_boost_added::ChatBoostAdded as Remote;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatBoostAdded {
    pub boost_count: u64,
}
impl From<Remote> for ChatBoostAdded {
    fn from(remote: Remote) -> Self {
        Self {
            boost_count: remote.boost_count,
        }
    }
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
