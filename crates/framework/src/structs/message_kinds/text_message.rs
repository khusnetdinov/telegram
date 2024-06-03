use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::link_preview_options::LinkPreviewOptions;
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::message_entity::MessageEntity;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextMessage {
    pub text: String,
    pub entities: Option<Vec<MessageEntity>>,
    pub link_preview_options: Option<LinkPreviewOptions>,
}

impl From<Inner> for TextMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            text,
            entities,
            link_preview_options,
            ..
        } = inner;

        Self {
            text: text.unwrap(),
            entities,
            link_preview_options,
        }
    }
}
