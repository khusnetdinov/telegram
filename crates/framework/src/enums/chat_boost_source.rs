use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::chat_boost_source::ChatBoostSource as Remote;
use telegram_bots_api::api::structs::chat_boost_source_gift_code::ChatBoostSourceGiftCode;
use telegram_bots_api::api::structs::chat_boost_source_giveaway::ChatBoostSourceGiveaway;
use telegram_bots_api::api::structs::chat_boost_source_premium::ChatBoostSourcePremium;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChatBoostSource {
    Premium(ChatBoostSourcePremium),
    GiftCode(ChatBoostSourceGiftCode),
    Giveaway(ChatBoostSourceGiveaway),
}

impl From<Remote> for ChatBoostSource {
    fn from(remote: Remote) -> Self {
        match remote {
            Remote::Premium(premium) => Self::Premium(premium),
            Remote::GiftCode(gift_code) => Self::GiftCode(gift_code),
            Remote::Giveaway(giveaway) => Self::Giveaway(giveaway),
        }
    }
}
