use crate::enums::chat_boost_source::ChatBoostSource;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_boost::ChatBoost as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct ChatBoost {
    pub boost_id: String,
    pub add_date: i64,
    pub expiration_date: i64,
    pub source: ChatBoostSource,
}
