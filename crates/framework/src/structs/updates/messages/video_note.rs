use crate::structs::media::video_note::VideoNote as Media;
use crate::structs::messages::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoNote {
    pub video_note: Media,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
}

impl From<IncomingMessage> for VideoNote {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            video_note,
            media_group_id,
            has_media_spoiler,
            caption,
            caption_entities,
            show_caption_above_media,
            ..
        } = remote;

        Self {
            // TODO: #[remote(into)]
            video_note: video_note.unwrap().into(),
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
