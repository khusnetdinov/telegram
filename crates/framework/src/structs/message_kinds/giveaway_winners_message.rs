use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::giveaway_winners::GiveawayWinners;
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiveawayWinnersMessage {
    pub giveaway_winners: GiveawayWinners,
}

impl From<Inner> for GiveawayWinnersMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            giveaway_winners, ..
        } = inner;

        Self {
            giveaway_winners: giveaway_winners.unwrap(),
        }
    }
}
