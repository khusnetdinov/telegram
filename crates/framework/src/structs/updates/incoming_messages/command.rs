use crate::structs::messages::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Command {
    pub text: String,
    pub entities: Option<Vec<MessageEntity>>,
}

impl From<IncomingMessage> for Command {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage { text, entities, .. } = remote;

        Self {
            text: text.unwrap(),
            entities: entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
        }
    }
}
