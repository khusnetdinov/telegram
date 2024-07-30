use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::bot_name::BotName as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotName {
    pub name: String,
}
impl From<Remote> for BotName {
    fn from(remote: Remote) -> Self {
        Self { name: remote.name }
    }
}
