use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LeftChatMemberMessage {
    pub left_chat_member: User,
}

impl From<Message> for LeftChatMemberMessage {
    fn from(remote: Message) -> Self {
        let Message {
            left_chat_member, ..
        } = remote;

        Self {
            left_chat_member: left_chat_member.unwrap(),
        }
    }
}
