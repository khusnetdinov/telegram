use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::message_entity::MessageEntity;
use telegram_bots_api::api::structs::video_note::VideoNote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoNoteMessage {
    pub video_note: VideoNote,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
}

impl From<Inner> for VideoNoteMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            video_note,
            media_group_id,
            has_media_spoiler,
            caption,
            caption_entities,
            show_caption_above_media,
            ..
        } = inner;

        Self {
            video_note: video_note.unwrap(),
            media_group_id,
            has_media_spoiler,
            caption,
            caption_entities,
            show_caption_above_media,
        }
    }
}
