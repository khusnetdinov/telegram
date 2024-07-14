use crate::structs::shared_user::SharedUser;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::users_shared::UsersShared as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsersShared {
    pub request_id: i64,
    pub user_ids: Vec<SharedUser>,
}
impl From<Remote> for UsersShared {
    fn from(remote: Remote) -> Self {
        Self {
            request_id: remote.request_id,
            // TODO: #[remote(map, into)]
            user_ids: remote
                .user_ids
                .iter()
                .map(|inner| inner.to_owned().into())
                .collect(),
        }
    }
}

impl From<Message> for UsersShared {
    fn from(remote: Message) -> Self {
        let Message {
            users_shared: Some(users_shared),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(users_shared)
    }
}
