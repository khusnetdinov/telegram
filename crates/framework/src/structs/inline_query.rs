use telegram_bots_api::api::structs::inline_query::InlineQuery as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, DerefInner, FromInner)]
pub struct InlineQuery {
    inner: Inner,
}
