use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::game::Game as Remote;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Game {
    pub game: Remote,
}

impl From<Message> for Game {
    fn from(remote: Message) -> Self {
        let Message { game, .. } = remote;

        Self {
            game: game.unwrap(),
        }
    }
}
