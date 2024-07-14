use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MigrateFromChat {
    pub migrate_from_chat_id: i64,
}

impl From<Message> for MigrateFromChat {
    fn from(remote: Message) -> Self {
        let Message {
            migrate_from_chat_id,
            ..
        } = remote;

        Self {
            migrate_from_chat_id: migrate_from_chat_id.unwrap(),
        }
    }
}
