use crate::structs::chat::Chat;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::story::Story as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Story {
    pub chat: Box<Chat>,
    pub id: i64,
}
impl From<Remote> for Story {
    fn from(remote: Remote) -> Self {
        Self {
            chat: Box::new((*remote.chat).into()),
            id: remote.id,
        }
    }
}
