use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::web_app_data::WebAppData as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct WebAppData {
    pub data: String,
    pub button_text: String,
}

impl From<Message> for WebAppData {
    fn from(remote: Message) -> Self {
        let Message { web_app_data, .. } = remote;

        Self::from(web_app_data.unwrap())
    }
}
