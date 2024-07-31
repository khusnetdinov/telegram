use crate::feature::web_app::WebAppInfo;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::menu_button_web_app::MenuButtonWebApp as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MenuButtonWebApp {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub text: String,
    pub web_app: WebAppInfo,
}
impl From<Remote> for MenuButtonWebApp {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            text: remote.text,
            web_app: remote.web_app.into(),
        }
    }
}
