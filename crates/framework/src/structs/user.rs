use telegram_macros::{DerefInner, FromInner};
use telegram_bots_api::api::structs::user::User as Inner;

#[derive(Debug, DerefInner, FromInner)]
pub struct User {
    pub inner: Inner,
}