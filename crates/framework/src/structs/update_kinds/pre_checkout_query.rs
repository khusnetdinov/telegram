use telegram_bots_api::api::structs::pre_checkout_query::PreCheckoutQuery as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, DerefInner, FromInner)]
pub struct PreCheckoutQuery {
    inner: Inner,
}
