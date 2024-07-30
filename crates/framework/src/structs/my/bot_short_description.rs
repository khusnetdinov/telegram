use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::bot_short_description::BotShortDescription as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotShortDescription {
    pub short_description: String,
}
impl From<Remote> for BotShortDescription {
    fn from(remote: Remote) -> Self {
        Self {
            short_description: remote.short_description,
        }
    }
}
