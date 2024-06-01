use telegram_bots_api::api::structs::business_message_deleted::BusinessMessagesDeleted as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, DerefInner, FromInner)]
pub struct BusinessMessagesDeleted {
    inner: Inner,
}
