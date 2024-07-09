use telegram_bots_api::api::structs::chat_member_updated::ChatMemberUpdated as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, Clone, DerefInner, FromInner)]
pub struct ChatMemberUpdated {
    inner: Inner,
}
