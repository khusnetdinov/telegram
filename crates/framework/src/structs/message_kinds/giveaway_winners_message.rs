use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::giveaway_winners::GiveawayWinners;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiveawayWinnersMessage {
    pub giveaway_winners: GiveawayWinners,
}

impl From<Message> for GiveawayWinnersMessage {
    fn from(inner: Message) -> Self {
        let Message {
            giveaway_winners, ..
        } = inner;

        Self {
            giveaway_winners: giveaway_winners.unwrap(),
        }
    }
}
