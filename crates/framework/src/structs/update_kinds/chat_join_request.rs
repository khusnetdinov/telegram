use telegram_bots_api::api::structs::chat_join_request::ChatJoinRequest as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, DerefInner, FromInner)]
pub struct ChatJoinRequest {
    inner: Inner,
}
