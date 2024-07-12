use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::link_preview_options::LinkPreviewOptions;
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::message_entity::MessageEntity;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextMessage {
    pub text: String,
    pub entities: Option<Vec<MessageEntity>>,
    pub link_preview_options: Option<LinkPreviewOptions>,
}

impl From<Message> for TextMessage {
    fn from(remote: Message) -> Self {
        let Message {
            text,
            entities,
            link_preview_options,
            ..
        } = remote;

        Self {
            text: text.unwrap(),
            entities,
            link_preview_options,
        }
    }
}
