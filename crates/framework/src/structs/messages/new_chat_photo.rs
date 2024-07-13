use crate::structs::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewChatPhotoMessage {
    pub new_chat_photo: Vec<PhotoSize>,
}

impl From<Message> for NewChatPhotoMessage {
    fn from(remote: Message) -> Self {
        let Message {
            new_chat_photo: Some(new_chat_photo),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self {
            // TODO: #[remote(map, into)]
            new_chat_photo: new_chat_photo
                .iter()
                .map(|inner| inner.to_owned().into())
                .collect(),
        }
    }
}
