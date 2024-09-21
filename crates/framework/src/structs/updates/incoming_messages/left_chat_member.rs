use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;
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

impl From<IncomingMessage> for LeftChatMember {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            left_chat_member, ..
        } = remote;

        Self::from(left_chat_member.unwrap())
    }
}
