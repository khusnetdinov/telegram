use crate::structs::media::animation::Animation as Media;
use crate::structs::media::document::Document;
use crate::structs::messages::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Animation {
    pub animation: Media,
    pub document: Document,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
}

impl From<IncomingMessage> for Animation {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
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
            // TODO: #[remote(into)]
            animation: animation.unwrap().into(),
            // TODO: #[remote(into)]
            document: document.unwrap().into(),
            media_group_id,
            has_media_spoiler,
            caption,
            // TODO: #[remote(option, map, into)]
            caption_entities: caption_entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            show_caption_above_media,
        }
    }
}
