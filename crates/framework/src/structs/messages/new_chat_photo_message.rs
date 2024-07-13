use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::photo_size::PhotoSize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewChatPhotoMessage {
    pub new_chat_photo: Vec<PhotoSize>,
}

impl From<Message> for NewChatPhotoMessage {
    fn from(remote: Message) -> Self {
        let Message { new_chat_photo, .. } = remote;

        Self {
            new_chat_photo: new_chat_photo.unwrap(),
        }
    }
}
