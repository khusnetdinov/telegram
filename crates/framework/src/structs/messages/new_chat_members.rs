use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewChatMembersMessage {
    pub new_chat_members: Vec<User>,
}

impl From<Message> for NewChatMembersMessage {
    fn from(remote: Message) -> Self {
        let Message {
            new_chat_members: Some(new_chat_members),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self {
            // TODO: #[remote(map, into)]
            new_chat_members: new_chat_members
                .iter()
                .map(|inner| inner.to_owned().into())
                .collect(),
        }
    }
}
