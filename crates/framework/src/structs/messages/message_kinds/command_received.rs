use crate::structs::messages::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandReceived {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
}

impl From<IncomingMessage> for CommandReceived {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage { text, entities, .. } = remote;

        Self {
            text: text.unwrap(),
            entities: entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
        }
    }
}
