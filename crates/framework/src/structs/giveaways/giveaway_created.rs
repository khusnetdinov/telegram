use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::giveaway_created::GiveawayCreated as Remote;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiveawayCreated {}

impl From<Remote> for GiveawayCreated {
    fn from(_remote: Remote) -> Self {
        Self {}
    }
}

impl From<Message> for GiveawayCreated {
    fn from(remote: Message) -> Self {
        let Message {
            giveaway_created: Some(giveaway_created),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(giveaway_created)
    }
}
