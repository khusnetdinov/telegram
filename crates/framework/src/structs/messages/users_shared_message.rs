use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::users_shared::UsersShared;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsersSharedMessage {
    pub users_shared: UsersShared,
}

impl From<Message> for UsersSharedMessage {
    fn from(remote: Message) -> Self {
        let Message { users_shared, .. } = remote;

        Self {
            users_shared: users_shared.unwrap(),
        }
    }
}
