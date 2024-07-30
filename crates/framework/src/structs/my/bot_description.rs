use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::bot_description::BotDescription as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotDescription {
    pub description: String,
}
impl From<Remote> for BotDescription {
    fn from(remote: Remote) -> Self {
        Self {
            description: remote.description,
        }
    }
}
