use crate::structs::chat::Chat;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::business_message_deleted::BusinessMessagesDeleted as Remote;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessMessagesDeleted {
    pub business_connection_id: String,
    pub chat: Chat,
    pub message_ids: Vec<i32>,
}

impl From<Remote> for BusinessMessagesDeleted {
    fn from(remote: Remote) -> Self {
        Self {
            business_connection_id: remote.business_connection_id,
            // TODO: #[remote(into)]
            chat: remote.chat.into(),
            message_ids: remote.message_ids,
        }
    }
}
