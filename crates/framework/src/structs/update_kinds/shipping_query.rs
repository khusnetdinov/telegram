use telegram_bots_api::api::structs::shipping_query::ShippingQuery as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, Clone, DerefInner, FromInner)]
pub struct ShippingQuery {
    inner: Inner,
}
