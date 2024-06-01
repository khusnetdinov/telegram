use telegram_bots_api::api::structs::business_connection::BusinessConnection as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, DerefInner, FromInner)]
pub struct BusinessConnection {
    inner: Inner,
}
