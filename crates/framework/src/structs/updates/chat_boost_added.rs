use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_boost_added::ChatBoostAdded as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct ChatBoostAdded {
    pub boost_count: u64,
}
