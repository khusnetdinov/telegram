use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message_id::MessageId as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageId {
    pub message_id: i64,
}

impl From<i64> for MessageId {
    fn from(message_id: i64) -> Self {
        Self { message_id }
    }
}

impl From<Remote> for MessageId {
    fn from(remote: Remote) -> Self {
        Self {
            message_id: remote.message_id,
        }
    }
}
