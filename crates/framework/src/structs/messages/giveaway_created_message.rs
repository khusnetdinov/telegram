use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::giveaway_created::GiveawayCreated;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiveawayCreatedMessage {
    pub giveaway_created: GiveawayCreated,
}

impl From<Message> for GiveawayCreatedMessage {
    fn from(remote: Message) -> Self {
        let Message {
            giveaway_created, ..
        } = remote;

        Self {
            giveaway_created: giveaway_created.unwrap(),
        }
    }
}
