use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewChatTitle {
    pub new_chat_title: String,
}

impl From<Message> for NewChatTitle {
    fn from(remote: Message) -> Self {
        let Message { new_chat_title, .. } = remote;

        Self {
            new_chat_title: new_chat_title.unwrap(),
        }
    }
}
