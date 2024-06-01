use telegram_bots_api::api::structs::chat_boost_removed::ChatBoostRemoved as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, DerefInner, FromInner)]
pub struct ChatBoostRemoved {
    inner: Inner,
}
