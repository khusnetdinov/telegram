use crate::structs::chat_boost_sources::chat_boost_source_gift_code::ChatBoostSourceGiftCode;
use crate::structs::chat_boost_sources::chat_boost_source_giveaway::ChatBoostSourceGiveaway;
use crate::structs::chat_boost_sources::chat_boost_source_premium::ChatBoostSourcePremium;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::chat_boost_source::ChatBoostSource as Remote;
use telegram_macros::FromRemoteEnum;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum)]
pub enum ChatBoostSource {
    Premium(ChatBoostSourcePremium),
    GiftCode(ChatBoostSourceGiftCode),
    Giveaway(ChatBoostSourceGiveaway),
}

impl Default for ChatBoostSource {
    fn default() -> Self {
        Self::Premium(ChatBoostSourcePremium {
            ..Default::default()
        })
    }
}
