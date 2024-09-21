use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;
use telegram_bots_api::api::structs::write_access_allowed::WriteAccessAllowed as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct WriteAccessAllowed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_attachment_menu: Option<bool>,
}

impl From<IncomingMessage> for WriteAccessAllowed {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            write_access_allowed,
            ..
        } = remote;

        Self::from(write_access_allowed.unwrap())
    }
}
