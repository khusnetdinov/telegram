use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::web_app_info::WebAppInfo as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct WebAppInfo {
    pub url: String,
}

impl From<WebAppInfo> for Remote {
    fn from(value: WebAppInfo) -> Self {
        Self { url: value.url }
    }
}
