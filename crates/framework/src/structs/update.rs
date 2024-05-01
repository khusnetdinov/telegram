use telegram_bots_api::api::structs::update::Update as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, DerefInner, FromInner)]
pub struct Update {
    pub inner: Inner,
}
