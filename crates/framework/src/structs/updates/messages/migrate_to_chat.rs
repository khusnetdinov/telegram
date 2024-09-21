use crate::enums::chat_uid::ChatUId;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MigrateToChat {
    pub migrate_to_chat_id: ChatUId,
}

impl From<Message> for MigrateToChat {
    fn from(remote: Message) -> Self {
        let Message {
            migrate_to_chat_id, ..
        } = remote;

        Self {
            migrate_to_chat_id: migrate_to_chat_id.unwrap().into(),
        }
    }
}
