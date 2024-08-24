use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_boost_source_gift_code::ChatBoostSourceGiftCode as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct ChatBoostSourceGiftCode {
    pub source: String,
    pub user: User,
}
