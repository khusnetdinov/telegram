use telegram_bots_api::api::structs::poll_answer::PollAnswer as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, DerefInner, FromInner)]
pub struct PollAnswer {
    inner: Inner,
}
