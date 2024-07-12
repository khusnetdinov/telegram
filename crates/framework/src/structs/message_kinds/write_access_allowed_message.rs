use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::write_access_allowed::WriteAccessAllowed;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WriteAccessAllowedMessage {
    pub write_access_allowed: WriteAccessAllowed,
}

impl From<Message> for WriteAccessAllowedMessage {
    fn from(remote: Message) -> Self {
        let Message {
            write_access_allowed,
            ..
        } = remote;

        Self {
            write_access_allowed: write_access_allowed.unwrap(),
        }
    }
}
