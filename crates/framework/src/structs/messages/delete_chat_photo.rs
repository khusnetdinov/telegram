use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteChatPhoto {
    pub delete_chat_photo: bool,
}

impl From<Message> for DeleteChatPhoto {
    fn from(remote: Message) -> Self {
        let Message {
            delete_chat_photo, ..
        } = remote;

        Self {
            delete_chat_photo: delete_chat_photo.unwrap(),
        }
    }
}
