use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::web_app_data::WebAppData;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebAppDataMessage {
    pub web_app_data: WebAppData,
}

impl From<Message> for WebAppDataMessage {
    fn from(remote: Message) -> Self {
        let Message { web_app_data, .. } = remote;

        Self {
            web_app_data: web_app_data.unwrap(),
        }
    }
}
