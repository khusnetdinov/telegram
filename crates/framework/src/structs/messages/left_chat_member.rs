use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::user::User as Remote;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LeftChatMember {
    pub left_chat_member: User,
}

impl From<Remote> for LeftChatMember {
    fn from(remote: Remote) -> Self {
        Self {
            left_chat_member: remote.into(),
        }
    }
}

impl From<Message> for LeftChatMember {
    fn from(remote: Message) -> Self {
        let Message {
            left_chat_member: Some(left_chat_member),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(left_chat_member)
    }
}
