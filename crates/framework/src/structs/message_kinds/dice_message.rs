use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::dice::Dice;
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiceMessage {
    pub dice: Dice,
}

impl From<Inner> for DiceMessage {
    fn from(inner: Inner) -> Self {
        let Inner { dice, .. } = inner;

        Self {
            dice: dice.unwrap(),
        }
    }
}
