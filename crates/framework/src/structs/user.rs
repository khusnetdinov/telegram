use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::user::User as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, Serialize, Deserialize, DerefInner, FromInner)]
pub struct User {
    pub inner: Inner,
}
