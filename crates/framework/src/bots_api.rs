use crate::config::Config;
use crate::structs::bot_command::BotCommand;
use crate::structs::update::Update;
use crate::structs::webhook::Webhook;
use crate::traits::bots_api::Pooler;
use std::thread::sleep;
use std::time::Duration;
use telegram_bots_api::api::params::delete_my_commands::DeleteMyCommands;
use telegram_bots_api::api::params::delete_webhook::DeleteWebhook;
use telegram_bots_api::api::params::get_my_commands::GetMyCommands;
use telegram_bots_api::api::params::get_update::GetUpdate;
use telegram_bots_api::api::params::set_my_commands::SetMyCommands;
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
}

impl BotsApi {
    pub fn commands(&self, params: (DeleteMyCommands, GetMyCommands, SetMyCommands)) {
        let (delete_params, _, set_params) = params;

        self.delete_commands(delete_params);
        self.set_commands(set_params);
    }

    pub fn delete_commands(&self, params: DeleteMyCommands) -> bool {
        self.client.delete_my_commands(&params).unwrap()
    }

    pub fn get_commands(&self, params: GetMyCommands) -> Vec<BotCommand> {
        self.client
            .get_my_commands(&params)
            .unwrap()
            .iter()
            .map(|inner| BotCommand::from(inner.to_owned()))
            .collect::<Vec<BotCommand>>()
    }

    pub fn set_commands(&self, params: SetMyCommands) -> bool {
        self.client.set_my_commands(&params).unwrap()
    }
}

impl From<Config> for BotsApi {
    fn from(config: Config) -> Self {
        let client = Sync::from(&*config);
        let webhook = Webhook::from(&config);
        let user = client.get_me().unwrap();

        Self::new(config, client, webhook, user)
    }
}

impl Pooler for BotsApi {
    fn pooling(&self, drop_pending_updates: bool, callback: impl Fn(&BotsApi, Update)) {
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
}
