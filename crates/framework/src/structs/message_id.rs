use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message_id::MessageId as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct MessageId {
    pub message_id: i64,
}

impl From<i64> for MessageId {
    fn from(message_id: i64) -> Self {
        Self { message_id }
    }
}
