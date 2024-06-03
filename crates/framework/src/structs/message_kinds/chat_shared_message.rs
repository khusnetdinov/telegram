use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_shared::ChatShared;
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatSharedMessage {
    pub chat_shared: ChatShared,
}

impl From<Inner> for ChatSharedMessage {
    fn from(inner: Inner) -> Self {
        let Inner { chat_shared, .. } = inner;

        Self {
            chat_shared: chat_shared.unwrap(),
        }
    }
}
