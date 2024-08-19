use crate::feature::audio::FileInput;
use crate::structs::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::input_media_video::InputMediaVideo as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InputMediaVideo {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub media: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<FileInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
}

impl From<Remote> for InputMediaVideo {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            media: remote.media,
            // TODO: #[remote(option, into)]
            thumbnail: remote.thumbnail.map(|inner| inner.into()),
            caption: remote.caption,
            parse_mode: remote.parse_mode,
            // TODO: #[remote(option, map, into)]
            caption_entities: remote
                .caption_entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            width: remote.width,
            height: remote.height,
            duration: remote.duration,
            has_spoiler: remote.has_spoiler,
            show_caption_above_media: remote.show_caption_above_media,
            supports_streaming: remote.supports_streaming,
        }
    }
}

impl From<InputMediaVideo> for Remote {
    fn from(value: InputMediaVideo) -> Self {
        Self {
            kind: value.kind,
            media: value.media,
            // TODO: #[value(option, into)]
            thumbnail: value.thumbnail.map(|inner| inner.into()),
            caption: value.caption,
            parse_mode: value.parse_mode,
            // TODO: #[value(option, map, into)]
            caption_entities: value
                .caption_entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            width: value.width,
            height: value.height,
            duration: value.duration,
            has_spoiler: value.has_spoiler,
            show_caption_above_media: value.show_caption_above_media,
            supports_streaming: value.supports_streaming,
        }
    }
}
