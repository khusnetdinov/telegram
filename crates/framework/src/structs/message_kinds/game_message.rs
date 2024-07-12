use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::game::Game;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameMessage {
    pub game: Game,
}

impl From<Message> for GameMessage {
    fn from(remote: Message) -> Self {
        let Message { game, .. } = remote;

        Self {
            game: game.unwrap(),
        }
    }
}
