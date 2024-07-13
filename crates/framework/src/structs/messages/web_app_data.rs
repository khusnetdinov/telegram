use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::web_app_data::WebAppData as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebAppData {
    pub data: String,
    pub button_text: String,
}
impl From<Remote> for WebAppData {
    fn from(remote: Remote) -> Self {
        Self {
            data: remote.data,
            button_text: remote.button_text,
        }
    }
}

impl From<Message> for WebAppData {
    fn from(remote: Message) -> Self {
        let Message {
            web_app_data: Some(web_app_data),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(web_app_data)
    }
}
