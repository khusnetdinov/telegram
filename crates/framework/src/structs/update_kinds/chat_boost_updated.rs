use telegram_bots_api::api::structs::chat_boost_updated::ChatBoostUpdated as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, Clone, DerefInner, FromInner)]
pub struct ChatBoostUpdated {
    inner: Inner,
}
