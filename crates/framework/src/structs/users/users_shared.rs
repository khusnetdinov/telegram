use crate::structs::shared_user::SharedUser;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::users_shared::UsersShared as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct UsersShared {
    pub request_id: i64,
    pub user_ids: Vec<SharedUser>,
}
