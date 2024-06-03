use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::giveaway_completed::GiveawayCompleted;
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiveawayCompletedMessage {
    pub giveaway_completed: GiveawayCompleted,
}

impl From<Inner> for GiveawayCompletedMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            giveaway_completed, ..
        } = inner;

        Self {
            giveaway_completed: giveaway_completed.unwrap(),
        }
    }
}
