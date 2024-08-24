use crate::feature::web_app::WebAppInfo;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::menu_button::MenuButton as Remote;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum MenuButton {
    Commands,
    WebApp { text: String, web_app: WebAppInfo },
    Default,
}

impl From<Remote> for MenuButton {
    fn from(remote: Remote) -> Self {
        match remote {
            Remote::Commands => Self::Commands,
            Remote::WebApp { text, web_app } => Self::WebApp {
                text,
                web_app: web_app.into(),
            },
            Remote::Default => Self::Default,
        }
    }
}

impl From<MenuButton> for Remote {
    fn from(value: MenuButton) -> Self {
        match value {
            MenuButton::Commands => Self::Commands,
            MenuButton::WebApp { text, web_app } => Self::WebApp {
                text,
                web_app: web_app.into(),
            },
            MenuButton::Default => Self::Default,
        }
    }
}
