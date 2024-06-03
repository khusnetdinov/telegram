use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewChatMembersMessage {
    pub new_chat_members: Vec<User>,
}

impl From<Inner> for NewChatMembersMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            new_chat_members, ..
        } = inner;

        Self {
            new_chat_members: new_chat_members.unwrap(),
        }
    }
}
