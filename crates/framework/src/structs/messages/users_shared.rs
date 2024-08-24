use crate::structs::shared_user::SharedUser;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::users_shared::UsersShared as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct UsersShared {
    pub request_id: i64,
    pub user_ids: Vec<SharedUser>,
}

impl From<Message> for UsersShared {
    fn from(remote: Message) -> Self {
        let Message { users_shared, .. } = remote;

        Self::from(users_shared.unwrap())
    }
}
