use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::giveaway::Giveaway;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiveawayMessage {
    pub giveaway: Giveaway,
}

impl From<Message> for GiveawayMessage {
    fn from(remote: Message) -> Self {
        let Message { giveaway, .. } = remote;

        Self {
            giveaway: giveaway.unwrap(),
        }
    }
}
