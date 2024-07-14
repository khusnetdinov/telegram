use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::write_access_allowed::WriteAccessAllowed as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WriteAccessAllowed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_attachment_menu: Option<bool>,
}
impl From<Remote> for WriteAccessAllowed {
    fn from(remote: Remote) -> Self {
        Self {
            from_request: remote.from_request,
            web_app_name: remote.web_app_name,
            from_attachment_menu: remote.from_attachment_menu,
        }
    }
}
impl From<Message> for WriteAccessAllowed {
    fn from(remote: Message) -> Self {
        let Message {
            write_access_allowed: Some(write_access_allowed),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(write_access_allowed)
    }
}
