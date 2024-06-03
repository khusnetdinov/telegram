use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat_background::ChatBackground;
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatBackgroundMessage {
    pub chat_background_set: ChatBackground,
}

impl From<Inner> for ChatBackgroundMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            chat_background_set,
            ..
        } = inner;

        Self {
            chat_background_set: chat_background_set.unwrap(),
        }
    }
}
