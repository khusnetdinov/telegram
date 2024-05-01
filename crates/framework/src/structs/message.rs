use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, DerefInner, FromInner)]
pub struct Message {
    inner: Inner,
}
