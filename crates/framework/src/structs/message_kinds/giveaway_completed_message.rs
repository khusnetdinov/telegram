use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::giveaway_completed::GiveawayCompleted;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiveawayCompletedMessage {
    pub giveaway_completed: GiveawayCompleted,
}

impl From<Message> for GiveawayCompletedMessage {
    fn from(remote: Message) -> Self {
        let Message {
            giveaway_completed, ..
        } = remote;

        Self {
            giveaway_completed: giveaway_completed.unwrap(),
        }
    }
}
