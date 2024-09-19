use crate::structs::chat::Chat;
use crate::structs::chats::chat_boost::ChatBoost;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_boost_updated::ChatBoostUpdated as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct ChatBoostUpdated {
    pub chat: Chat,
    pub boost: ChatBoost,
}
