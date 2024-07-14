use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::dice::Dice as Remote;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dice {
    pub emoji: String,
    pub value: i64,
}
impl From<Remote> for Dice {
    fn from(remote: Remote) -> Self {
        Self {
            emoji: remote.emoji,
            value: remote.value,
        }
    }
}
impl From<Message> for Dice {
    fn from(remote: Message) -> Self {
        let Message {
            dice: Some(dice), ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(dice)
    }
}
