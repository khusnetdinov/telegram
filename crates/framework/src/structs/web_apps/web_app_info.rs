use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::web_app_info::WebAppInfo as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebAppInfo {
    pub url: String,
}
impl From<Remote> for WebAppInfo {
    fn from(remote: Remote) -> Self {
        Self { url: remote.url }
    }
}

impl From<WebAppInfo> for Remote {
    fn from(value: WebAppInfo) -> Self {
        Self { url: value.url }
    }
}
