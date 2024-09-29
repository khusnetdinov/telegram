use crate::structs::link_preview_options::LinkPreviewOptions;
use crate::structs::messages::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextReceived {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
}

impl From<IncomingMessage> for TextReceived {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
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
