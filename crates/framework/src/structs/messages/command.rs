use crate::structs::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Command {
    pub text: String,
    pub entities: Option<Vec<MessageEntity>>,
}

impl From<Message> for Command {
    fn from(remote: Message) -> Self {
        let Message { text, entities, .. } = remote;

        Self {
            text: text.unwrap(),
            entities: entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
        }
    }
}
