use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::bot_command::BotCommand as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}

impl From<Remote> for BotCommand {
    fn from(remote: Remote) -> Self {
        Self {
            command: remote.command,
            description: remote.description,
        }
    }
}
