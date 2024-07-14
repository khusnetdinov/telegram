use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChannelChatCreated {
    pub channel_chat_created: bool,
}

impl From<Message> for ChannelChatCreated {
    fn from(remote: Message) -> Self {
        let Message {
            channel_chat_created,
            ..
        } = remote;

        Self {
            channel_chat_created: channel_chat_created.unwrap(),
        }
    }
}
