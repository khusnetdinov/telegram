use crate::structs::link_preview_options::LinkPreviewOptions;
use crate::structs::messages::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Text {
    pub text: String,
    pub entities: Option<Vec<MessageEntity>>,
    pub link_preview_options: Option<LinkPreviewOptions>,
}

impl From<Message> for Text {
    fn from(remote: Message) -> Self {
        let Message {
            text,
            entities,
            link_preview_options,
            ..
        } = remote;

        Self {
            text: text.unwrap(),
            entities: entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            link_preview_options: link_preview_options.map(|inner| inner.into()),
        }
    }
}
