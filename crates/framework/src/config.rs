use telegram_bots_api::api::structs::input_file::InputFile;
use telegram_bots_api::config::Config as Inner;
use telegram_macros::DerefInner;

#[derive(Debug, DerefInner)]
pub struct Config {
    pub inner: Inner,
    // TODO: begin:webhook
    pub drop_pending_updates: Option<bool>,
    pub certificate: Option<InputFile>,
    pub secret_token: Option<String>,
    // TODO: end:webhook
    // TODO: begin:pooling
    pub pooling_timeout: Option<u64>,
    // TODO: end:pooling
}

impl Config {
    pub fn new() -> Self {
        Self {
            inner: Inner::new(),
            drop_pending_updates: Some(true),
            certificate: None,
            secret_token: None,
            pooling_timeout: Some(1),
        }
    }
}
