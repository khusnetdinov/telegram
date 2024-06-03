use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::game::Game;
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameMessage {
    pub game: Game,
}

impl From<Inner> for GameMessage {
    fn from(inner: Inner) -> Self {
        let Inner { game, .. } = inner;

        Self {
            game: game.unwrap(),
        }
    }
}
