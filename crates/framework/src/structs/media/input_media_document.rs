use crate::feature::audio::FileInput;
use crate::structs::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::input_media_document::InputMediaDocument as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InputMediaDocument {
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
    pub disable_content_type_detection: Option<bool>,
}

impl From<Remote> for InputMediaDocument {
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
            disable_content_type_detection: remote.disable_content_type_detection,
        }
    }
}

impl From<InputMediaDocument> for Remote {
    fn from(value: InputMediaDocument) -> Self {
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
            disable_content_type_detection: value.disable_content_type_detection,
        }
    }
}
