use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::dice::Dice as Remote;
use telegram_bots_api::api::structs::message::Message as IncomingMessage;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct Dice {
    pub emoji: String,
    pub value: i64,
}

impl From<IncomingMessage> for Dice {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage { dice, .. } = remote;

        Self::from(dice.unwrap())
    }
}
