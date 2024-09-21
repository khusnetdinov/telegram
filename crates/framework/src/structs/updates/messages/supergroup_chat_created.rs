use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SupergroupChatCreated {
    pub supergroup_chat_created: bool,
}

impl From<IncomingMessage> for SupergroupChatCreated {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            supergroup_chat_created,
            ..
        } = remote;

        Self {
            supergroup_chat_created: supergroup_chat_created.unwrap(),
        }
    }
}
