use telegram_bots_api::api::requests::sync::Requests;
use telegram_bots_api::api::structs::user::User;
use crate::config::Config;
use crate::structs::webhook::Webhook;
use telegram_bots_api::clients::sync::Sync;

#[derive(Debug)]
pub struct BotsApi {
    config: Config,
    client: Sync,
    webhook: Webhook,
    user: User
}

impl From<Config> for BotsApi {
    fn from(config: Config) -> Self {
        let Config { ref inner } = config;
        let client = Sync::from(inner.clone());
        let webhook = Webhook::from(inner);
        let user = client.get_me().unwrap();

        Self {
            config,
            client,
            webhook,
            user
        }
    }
}

impl BotsApi {
    pub fn new() -> Self {
        let config = Config::new();

        Self::from(config)
    }

    pub fn pooling(&self, _drop_pending_updates: bool, _callback: fn()) {
        todo!()
    }

    pub fn listen_http(&self) {
        todo!()
    }

    pub fn listen_https(&self) {
        todo!()
    }
}
