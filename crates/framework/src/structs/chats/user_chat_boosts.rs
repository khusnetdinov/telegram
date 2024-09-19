use crate::structs::chats::chat_boost::ChatBoost;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::user_chat_boosts::UserChatBoosts as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct UserChatBoosts {
    pub boosts: Vec<ChatBoost>,
}
