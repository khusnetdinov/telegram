use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::animation::Animation;
use telegram_bots_api::api::structs::document::Document;
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::message_entity::MessageEntity;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnimationMessage {
    pub animation: Animation,
    pub document: Document,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
}

impl From<Message> for AnimationMessage {
    fn from(remote: Message) -> Self {
        let Message {
            animation,
            document,
            media_group_id,
            has_media_spoiler,
            caption,
            caption_entities,
            show_caption_above_media,
            ..
        } = remote;

        Self {
            animation: animation.unwrap(),
            document: document.unwrap(),
            media_group_id,
            has_media_spoiler,
            caption,
            caption_entities,
            show_caption_above_media,
        }
    }
}
