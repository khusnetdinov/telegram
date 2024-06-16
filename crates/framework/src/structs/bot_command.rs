use telegram_bots_api::api::structs::bot_command::BotCommand as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, DerefInner, FromInner)]
pub struct BotCommand {
    pub inner: Inner,
}
