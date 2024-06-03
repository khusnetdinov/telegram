use telegram_bots_api::api::structs::callback_query::CallbackQuery as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, DerefInner, FromInner)]
pub struct CallbackQuery {
    inner: Inner,
}
