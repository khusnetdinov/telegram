use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_boost_source_gift_code::ChatBoostSourceGiftCode as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct ChatBoostSourceGiftCode {
    pub source: String,
    pub user: User,
}
