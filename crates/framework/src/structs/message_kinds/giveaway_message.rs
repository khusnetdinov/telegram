use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::giveaway::Giveaway;
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiveawayMessage {
    pub giveaway: Giveaway,
}

impl From<Inner> for GiveawayMessage {
    fn from(inner: Inner) -> Self {
        let Inner { giveaway, .. } = inner;

        Self {
            giveaway: giveaway.unwrap(),
        }
    }
}
