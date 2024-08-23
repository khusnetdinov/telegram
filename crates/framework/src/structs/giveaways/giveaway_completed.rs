use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::giveaway_completed::GiveawayCompleted as Remote;
use telegram_bots_api::api::structs::message::Message;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct GiveawayCompleted {
    pub winner_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_completed: Option<Box<Self>>,
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
