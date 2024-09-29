use crate::structs::users::shared_user::SharedUser;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;
use telegram_bots_api::api::structs::users_shared::UsersShared as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct UsersShared {
    pub request_id: i64,
    pub user_ids: Vec<SharedUser>,
}
impl From<IncomingMessage> for UsersShared {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage { users_shared, .. } = remote;

        Self::from(users_shared.unwrap())
    }
}
