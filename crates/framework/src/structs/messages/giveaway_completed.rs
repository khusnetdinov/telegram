use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::giveaway_completed::GiveawayCompleted as Remote;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiveawayCompleted {
    pub winner_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_completed: Option<Box<Remote>>,
}
impl From<Remote> for GiveawayCompleted {
    fn from(remote: Remote) -> Self {
        Self {
            winner_count: remote.winner_count,
            unclaimed_prize_count: remote.unclaimed_prize_count,
            // TODO: #[remote(option, into)]
            giveaway_completed: remote.giveaway_completed.map(|inner| (*inner).into()),
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
