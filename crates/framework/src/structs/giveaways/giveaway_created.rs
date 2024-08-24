use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::giveaway_created::GiveawayCreated as Remote;
use telegram_bots_api::api::structs::message::Message;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct GiveawayCreated {}

impl From<Message> for GiveawayCreated {
    fn from(remote: Message) -> Self {
        let Message {
            giveaway_created, ..
        } = remote;

        Self::from(giveaway_created.unwrap())
    }
}
