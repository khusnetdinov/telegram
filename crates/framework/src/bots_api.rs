use crate::config::Config;

#[derive(Debug)]
pub struct BotsApi {
    /// Config
    config: Config,
}

impl From<Config> for BotsApi {
    fn from(config: Config) -> Self {
        Self { config }
    }
}

impl BotsApi {
    pub fn new() -> Self {
        let config = Config::new();

        Self { config }
    }

    pub fn pooling(&self, drop_pending_updates: bool, callback: fn()) {
        todo!()
    }

    pub fn listen_http(&self) {
        todo!()
    }

    pub fn listen_https(&self) {
        todo!()
    }
}
