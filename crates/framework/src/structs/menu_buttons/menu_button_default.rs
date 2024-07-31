use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::menu_button_default::MenuButtonDefault as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MenuButtonDefault {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
impl From<Remote> for MenuButtonDefault {
    fn from(remote: Remote) -> Self {
        Self { kind: remote.kind }
    }
}
