use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SupergroupChatCreated {
    pub supergroup_chat_created: bool,
}

impl From<Message> for SupergroupChatCreated {
    fn from(remote: Message) -> Self {
        let Message {
            supergroup_chat_created,
            ..
        } = remote;

        Self {
            supergroup_chat_created: supergroup_chat_created.unwrap(),
        }
    }
}
