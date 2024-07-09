use crate::structs::chat_boost_sources::chat_boost_source_gift_code::ChatBoostSourceGiftCode;
use crate::structs::chat_boost_sources::chat_boost_source_giveaway::ChatBoostSourceGiveaway;
use crate::structs::chat_boost_sources::chat_boost_source_premium::ChatBoostSourcePremium;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::chat_boost_source::ChatBoostSource as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChatBoostSource {
    Premium(ChatBoostSourcePremium),
    GiftCode(ChatBoostSourceGiftCode),
    Giveaway(ChatBoostSourceGiveaway),
}

impl From<Remote> for ChatBoostSource {
    fn from(remote: Remote) -> Self {
        match remote {
            Remote::Premium(premium) => Self::Premium(premium.into()),
            Remote::GiftCode(gift_code) => Self::GiftCode(gift_code.into()),
            Remote::Giveaway(giveaway) => Self::Giveaway(giveaway.into()),
        }
    }
}
