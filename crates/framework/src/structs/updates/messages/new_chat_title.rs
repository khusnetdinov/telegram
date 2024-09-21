use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewChatTitle {
    pub new_chat_title: String,
}

impl From<IncomingMessage> for NewChatTitle {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage { new_chat_title, .. } = remote;

        Self {
            new_chat_title: new_chat_title.unwrap(),
        }
    }
}
