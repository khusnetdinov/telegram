use std::thread::sleep;
use crate::structs::config::Config;
use telegram_bots_api::api::requests::sync::Requests;
use telegram_bots_api::api::types::user::User;
use telegram_bots_api::clients::sync::Sync;
use std::time::{Duration};
use telegram_bots_api::api::params::get_update::GetUpdate;
use telegram_bots_api::api::types::update::Update;

#[derive(Debug)]
pub struct BotsApi {
    client: Sync,
    user: User,
    config: telegram_bots_api::config::Config
}

impl From<Config> for BotsApi {
    fn from(config: Config) -> Self {
        let client = Sync::from(config.inner);
        let config = client.config.clone();
        let user = client.get_me().unwrap();

        Self { client, config, user }
    }
}

impl BotsApi {
    pub fn new() -> Self {
        let client = Sync::new();
        let config = client.config.clone();
        let user = client.get_me().unwrap();

        Self { client, config, user }
    }

    /// Pooling updates for telegram bots api. Pass callback `fn(updates: Vec<Update>)`
    pub fn pooling(&self, callback: fn(updates: Vec<Update>)) {
        loop {
            let params = GetUpdate {
                offset: self.config.updates_offset,
                limit: self.config.updates_limit,
                timeout: self.config.updates_timeout,
                allowed_updates: None
            };
            let updates = self.client.get_updates(&params).unwrap();

            callback(updates);
            sleep(Duration::from_secs(1).clone());
        }
    }

    pub fn listen_http(&self) {
        todo!()
    }

    pub fn listen_https(&self) {
        todo!()
    }
}
