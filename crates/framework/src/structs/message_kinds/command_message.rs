use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::message_entity::MessageEntity;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandMessage {
    pub text: String,
    pub entities: Option<Vec<MessageEntity>>,
}

impl From<Inner> for CommandMessage {
    fn from(inner: Inner) -> Self {
        let Inner { text, entities, .. } = inner;

        Self {
            text: text.unwrap(),
            entities,
        }
    }
}
