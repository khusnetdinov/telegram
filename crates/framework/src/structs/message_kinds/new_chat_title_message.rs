use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewChatTitleMessage {
    pub new_chat_title: String,
}

impl From<Inner> for NewChatTitleMessage {
    fn from(inner: Inner) -> Self {
        let Inner { new_chat_title, .. } = inner;

        Self {
            new_chat_title: new_chat_title.unwrap(),
        }
    }
}
