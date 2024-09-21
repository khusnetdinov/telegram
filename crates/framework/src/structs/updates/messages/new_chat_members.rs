use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewChatMembersMessage {
    pub new_chat_members: Vec<User>,
}

impl From<IncomingMessage> for NewChatMembersMessage {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            new_chat_members, ..
        } = remote;

        let new_chat_members = new_chat_members
            .unwrap()
            .iter()
            .map(|inner| inner.to_owned().into())
            .collect();

        Self { new_chat_members }
    }
}
