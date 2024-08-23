use crate::structs::chat::Chat;
use crate::structs::chat_boost::ChatBoost;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_boost_updated::ChatBoostUpdated as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct ChatBoostUpdated {
    pub chat: Chat,
    pub boost: ChatBoost,
}
