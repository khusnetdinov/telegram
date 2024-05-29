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
    config: Config,
    pub client: Sync,
    pub user: User,
    pub webhook: Webhook,
}

impl From<Config> for BotsApi {
    fn from(config: Config) -> Self {
        let client = Sync::from(&*config);
        let webhook = Webhook::from(&config);
        let user = client.get_me().unwrap();

        Self::new(config, client, webhook, user)
    }
}

impl BotsApi {
    pub fn new(config: Config, client: Sync, webhook: Webhook, user: User) -> Self {
        Self {
            config,
            client,
            webhook,
            user,
        }
    }
    pub fn from_env() -> Self {
        let config = Config::new();

        Self::from(config)
    }

    pub fn pooling(&self, drop_pending_updates: bool, callback: impl Fn(&BotsApi, Update)) {
        let mut update_offset = self.config.updates_offset;

        self.client
            .delete_webhook(&DeleteWebhook {
                drop_pending_updates: Some(drop_pending_updates),
            })
            .unwrap();

        loop {
            let updates = self
                .client
                .get_updates(&GetUpdate {
                    offset: update_offset,
                    limit: self.config.updates_limit,
                    timeout: self.config.updates_timeout,
                    allowed_updates: None,
                })
                .unwrap();

            for inner in updates.into_iter() {
                let offset = &inner.update_id + 1i64;

                callback(self, Update::from(inner));

                update_offset = offset;
            }

            sleep(Duration::from_secs(1));
        }
    }

    pub fn http_listen(&self) {
        todo!()
    }

    pub fn https_listen(&self) {
        todo!()
    }
}
