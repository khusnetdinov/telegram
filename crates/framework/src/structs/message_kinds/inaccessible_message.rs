use crate::structs::chat::Chat;
use crate::structs::message_id::MessageId;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::inaccessible_message::InaccessibleMessage as Remote;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InaccessibleMessage {
    pub chat: Chat,
    pub message_id: MessageId,
    pub date: i64,
}

impl From<Remote> for InaccessibleMessage {
    fn from(remote: Remote) -> Self {
        Self {
            // TODO: #[remote(into)]
            chat: remote.chat.into(),
            // TODO: #[remote(into)]
            message_id: remote.message_id.into(),
            date: remote.date,
        }
    }
}
