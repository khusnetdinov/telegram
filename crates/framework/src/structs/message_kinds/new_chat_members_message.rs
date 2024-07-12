use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewChatMembersMessage {
    pub new_chat_members: Vec<User>,
}

impl From<Message> for NewChatMembersMessage {
    fn from(remote: Message) -> Self {
        let Message {
            new_chat_members, ..
        } = remote;

        Self {
            new_chat_members: new_chat_members.unwrap(),
        }
    }
}
