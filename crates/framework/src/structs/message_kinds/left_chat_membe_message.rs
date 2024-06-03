use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LeftChatMemberMessage {
    pub left_chat_member: User,
}

impl From<Inner> for LeftChatMemberMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            left_chat_member, ..
        } = inner;

        Self {
            left_chat_member: left_chat_member.unwrap(),
        }
    }
}
