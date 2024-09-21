use crate::enums::chat_uid::ChatUId;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MigrateToChat {
    pub migrate_to_chat_id: ChatUId,
}

impl From<IncomingMessage> for MigrateToChat {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            migrate_to_chat_id, ..
        } = remote;

        Self {
            migrate_to_chat_id: migrate_to_chat_id.unwrap().into(),
        }
    }
}
