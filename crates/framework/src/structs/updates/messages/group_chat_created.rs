use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupChatCreated {
    pub group_chat_created: bool,
}

impl From<IncomingMessage> for GroupChatCreated {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            group_chat_created, ..
        } = remote;

        Self {
            group_chat_created: group_chat_created.unwrap(),
        }
    }
}
