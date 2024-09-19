use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::dice::Dice as Remote;
use telegram_bots_api::api::structs::message::Message;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct Dice {
    pub emoji: String,
    pub value: i64,
}

impl From<Message> for Dice {
    fn from(remote: Message) -> Self {
        let Message { dice, .. } = remote;

        Self::from(dice.unwrap())
    }
}
