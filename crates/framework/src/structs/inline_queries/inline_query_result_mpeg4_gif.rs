use crate::enums::input_message_content::InputMessageContent;
use crate::structs::message_entity::MessageEntity;
use crate::structs::reply_markups::inline_keyboard_markup::InlineKeyboardMarkup;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::inline_query_result_mpeg4_gif::InlineQueryResultMpeg4Gif as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct InlineQueryResultMpeg4Gif {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: String,
    pub mpeg4_url: String,
    pub thumbnail_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
}
