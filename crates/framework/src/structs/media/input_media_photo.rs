use crate::structs::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::input_media_photo::InputMediaPhoto as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InputMediaPhoto {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub media: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
}

impl From<Remote> for InputMediaPhoto {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            media: remote.media,
            caption: remote.caption,
            parse_mode: remote.parse_mode,
            // TODO: #[remote(option, map, into)]
            caption_entities: remote
                .caption_entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            has_spoiler: remote.has_spoiler,
            show_caption_above_media: remote.show_caption_above_media,
        }
    }
}

impl From<InputMediaPhoto> for Remote {
    fn from(value: InputMediaPhoto) -> Self {
        Self {
            kind: value.kind,
            media: value.media,
            caption: value.caption,
            parse_mode: value.parse_mode,
            // TODO: #[value(option, map, into)]
            caption_entities: value
                .caption_entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            has_spoiler: value.has_spoiler,
            show_caption_above_media: value.show_caption_above_media,
        }
    }
}
