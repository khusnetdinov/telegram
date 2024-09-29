use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatPhotoDeleted {
    pub delete_chat_photo: bool,
}

impl From<IncomingMessage> for ChatPhotoDeleted {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            delete_chat_photo, ..
        } = remote;

        Self {
            delete_chat_photo: delete_chat_photo.unwrap(),
        }
    }
}
