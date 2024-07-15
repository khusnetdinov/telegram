use crate::structs::updates::message::Message;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::message_result::MessageResult as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MessageResult {
    Inline(bool),
    Message(Message),
}

impl From<Remote> for MessageResult {
    fn from(remote: Remote) -> Self {
        match remote {
            Remote::Inline(value) => Self::Inline(value),
            Remote::Message(message) => Self::Message(message.into()),
        }
    }
}
