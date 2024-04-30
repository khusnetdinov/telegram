use telegram_bots_api::config::Config as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, DerefInner, FromInner)]
pub struct Config {
    pub inner: Inner,
}

impl Config {
    pub fn new() -> Self {
        Self {
            inner: Inner::new(),
        }
    }
}
