use std::rc::Rc;
use telegram_bots_api::config::Config as Inner;
use telegram_macros::DerefInner;

#[derive(Debug, DerefInner)]
pub struct Config {
    pub inner: Rc<Inner>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(Inner::new()),
        }
    }
}
