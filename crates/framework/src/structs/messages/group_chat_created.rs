use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupChatCreated {
    pub group_chat_created: bool,
}

impl From<Message> for GroupChatCreated {
    fn from(remote: Message) -> Self {
        let Message {
            group_chat_created, ..
        } = remote;

        Self {
            group_chat_created: group_chat_created.unwrap(),
        }
    }
}
