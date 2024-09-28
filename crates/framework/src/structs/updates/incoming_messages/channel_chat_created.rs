use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChannelChatCreated {
    pub channel_chat_created: bool,
}

impl From<IncomingMessage> for ChannelChatCreated {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            channel_chat_created,
            ..
        } = remote;

        Self {
            channel_chat_created: channel_chat_created.unwrap(),
        }
    }
}
