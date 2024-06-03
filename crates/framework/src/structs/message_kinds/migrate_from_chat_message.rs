use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MigrateFromChatMessage {
    pub migrate_from_chat_id: i64,
}

impl From<Inner> for MigrateFromChatMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            migrate_from_chat_id,
            ..
        } = inner;

        Self {
            migrate_from_chat_id: migrate_from_chat_id.unwrap(),
        }
    }
}
