use crate::config::Config;
use crate::structs::update::Update;
use crate::structs::webhook::Webhook;
use std::thread::sleep;
use std::time::Duration;
use telegram_bots_api::api::params::delete_webhook::DeleteWebhook;
use telegram_bots_api::api::params::get_update::GetUpdate;
use telegram_bots_api::api::requests::sync::Requests;
use telegram_bots_api::api::structs::user::User;
use telegram_bots_api::clients::sync::Sync;

#[derive(Debug)]
pub struct BotsApi {
    pub client: Sync,
    config: Config,
    webhook: Webhook,
    user: User,
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
            user,
        }
    }
}

impl BotsApi {
    pub fn new() -> Self {
        let config = Config::new();

        Self::from(config)
    }

    pub fn pooling(&self, drop_pending_updates: bool, callback: fn(updates: Update)) {
        self.client
            .delete_webhook(&DeleteWebhook {
                drop_pending_updates: Some(drop_pending_updates),
            })
            .unwrap();

        loop {
            let updates = self
                .client
                .get_updates(&GetUpdate {
                    offset: self.config.updates_offset,
                    limit: self.config.updates_limit,
                    timeout: self.config.updates_timeout,
                    allowed_updates: None,
                })
                .unwrap();

            for inner in updates.into_iter() {
                callback(Update { inner });
            }

            sleep(Duration::from_secs(1));
        }
    }

    pub fn listen_http(&self) {
        todo!()
    }

    pub fn listen_https(&self) {
        todo!()
    }
}
