use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteChatPhotoMessage {
    pub delete_chat_photo: bool,
}

impl From<Inner> for DeleteChatPhotoMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            delete_chat_photo, ..
        } = inner;

        Self {
            delete_chat_photo: delete_chat_photo.unwrap(),
        }
    }
}
