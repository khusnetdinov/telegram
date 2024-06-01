use telegram_bots_api::api::structs::chosen_inline_result::ChosenInlineResult as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, DerefInner, FromInner)]
pub struct ChosenInlineResult {
    inner: Inner,
}
