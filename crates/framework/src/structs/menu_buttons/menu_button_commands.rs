use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::menu_button_commands::MenuButtonCommands as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MenuButtonCommands {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
impl From<Remote> for MenuButtonCommands {
    fn from(remote: Remote) -> Self {
        Self { kind: remote.kind }
    }
}
