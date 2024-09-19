use crate::enums::chat_boost_source::ChatBoostSource;
use crate::structs::chat::Chat;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_boost_removed::ChatBoostRemoved as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct ChatBoostRemoved {
    pub chat: Chat,
    pub boost_id: String,
    pub remove_date: i64,
    pub source: ChatBoostSource,
}
