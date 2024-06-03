use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::users_shared::UsersShared;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsersSharedMessage {
    pub users_shared: UsersShared,
}

impl From<Inner> for UsersSharedMessage {
    fn from(inner: Inner) -> Self {
        let Inner { users_shared, .. } = inner;

        Self {
            users_shared: users_shared.unwrap(),
        }
    }
}
