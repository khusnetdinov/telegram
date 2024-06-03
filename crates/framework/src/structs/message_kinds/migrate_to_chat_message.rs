use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MigrateToChatMessage {
    pub migrate_to_chat_id: i64,
}

impl From<Inner> for MigrateToChatMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            migrate_to_chat_id, ..
        } = inner;

        Self {
            migrate_to_chat_id: migrate_to_chat_id.unwrap(),
        }
    }
}
