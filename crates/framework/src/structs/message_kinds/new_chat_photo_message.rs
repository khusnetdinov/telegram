use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::photo_size::PhotoSize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewChatPhotoMessage {
    pub new_chat_photo: Vec<PhotoSize>,
}

impl From<Inner> for NewChatPhotoMessage {
    fn from(inner: Inner) -> Self {
        let Inner { new_chat_photo, .. } = inner;

        Self {
            new_chat_photo: new_chat_photo.unwrap(),
        }
    }
}
