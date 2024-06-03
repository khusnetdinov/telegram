use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChannelChatCreatedMessage {
    pub channel_chat_created: bool,
}

impl From<Inner> for ChannelChatCreatedMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            channel_chat_created,
            ..
        } = inner;

        Self {
            channel_chat_created: channel_chat_created.unwrap(),
        }
    }
}
