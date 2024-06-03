use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::giveaway_created::GiveawayCreated;
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiveawayCreatedMessage {
    pub giveaway_created: GiveawayCreated,
}

impl From<Inner> for GiveawayCreatedMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            giveaway_created, ..
        } = inner;

        Self {
            giveaway_created: giveaway_created.unwrap(),
        }
    }
}
