use crate::enums::chat_uid::ChatUId;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MigrateFromChat {
    pub migrate_from_chat_id: ChatUId,
}

impl From<IncomingMessage> for MigrateFromChat {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            migrate_from_chat_id,
            ..
        } = remote;

        Self {
            migrate_from_chat_id: migrate_from_chat_id.unwrap().into(),
        }
    }
}
