use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::giveaway_completed::GiveawayCompleted as Remote;
use telegram_bots_api::api::structs::message::Message;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct GiveawayCompleted {
    pub winner_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_completed: Option<Box<Self>>,
}

impl From<GiveawayCompleted> for Remote {
    fn from(value: GiveawayCompleted) -> Self {
        Self {
            winner_count: value.winner_count,
            unclaimed_prize_count: value.unclaimed_prize_count,
            // TODO: #[remote(option, into)]
            giveaway_completed: value
                .giveaway_completed
                .map(|inner| Box::new((*inner).into())),
        }
    }
}

impl From<Message> for GiveawayCompleted {
    fn from(remote: Message) -> Self {
        let Message {
            giveaway_completed: Some(giveaway_completed),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(giveaway_completed)
    }
}
