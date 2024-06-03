use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupChatCreatedMessage {
    pub group_chat_created: bool,
}

impl From<Inner> for GroupChatCreatedMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            group_chat_created, ..
        } = inner;

        Self {
            group_chat_created: group_chat_created.unwrap(),
        }
    }
}
