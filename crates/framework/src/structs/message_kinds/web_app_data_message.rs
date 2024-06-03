use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::web_app_data::WebAppData;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebAppDataMessage {
    pub web_app_data: WebAppData,
}

impl From<Inner> for WebAppDataMessage {
    fn from(inner: Inner) -> Self {
        let Inner { web_app_data, .. } = inner;

        Self {
            web_app_data: web_app_data.unwrap(),
        }
    }
}
