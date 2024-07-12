use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::message_entity::MessageEntity;
use telegram_bots_api::api::structs::photo_size::PhotoSize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PhotoMessage {
    pub photo: Vec<PhotoSize>,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
}

impl From<Message> for PhotoMessage {
    fn from(remote: Message) -> Self {
        let Message {
            photo,
            media_group_id,
            has_media_spoiler,
            caption,
            caption_entities,
            show_caption_above_media,
            ..
        } = remote;

        Self {
            photo: photo.unwrap(),
            media_group_id,
            has_media_spoiler,
            caption,
            caption_entities,
            show_caption_above_media,
        }
    }
}
