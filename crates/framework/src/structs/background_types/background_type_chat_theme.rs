use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::background_type_chat_theme::BackgroundTypeChatTheme as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundTypeChatTheme {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub theme_name: String,
}

impl From<Remote> for BackgroundTypeChatTheme {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            theme_name: remote.theme_name,
        }
    }
}
