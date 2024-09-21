use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;
use telegram_bots_api::api::structs::web_app_data::WebAppData as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct WebAppData {
    pub data: String,
    pub button_text: String,
}

impl From<IncomingMessage> for WebAppData {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage { web_app_data, .. } = remote;

        Self::from(web_app_data.unwrap())
    }
}
