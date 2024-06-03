use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SupergroupChatCreatedMessage {
    pub supergroup_chat_created: bool,
}

impl From<Inner> for SupergroupChatCreatedMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            supergroup_chat_created,
            ..
        } = inner;

        Self {
            supergroup_chat_created: supergroup_chat_created.unwrap(),
        }
    }
}
