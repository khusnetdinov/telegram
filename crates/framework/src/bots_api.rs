use crate::config::Config;
use crate::structs::bot_command::BotCommand;
use crate::structs::update::Update;
use crate::structs::user::User;
use crate::structs::webhook::Webhook;
use crate::structs::webhook_info::WebhookInfo;
use crate::traits::bots_api::Commander;
use crate::traits::bots_api::Pooler;
use crate::traits::bots_api::Sender;
use crate::traits::bots_api::Webhooker;
use crate::traits::params::Params;

use std::thread::sleep;
use std::time::Duration;
use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::params::delete_my_commands::DeleteMyCommands;
use telegram_bots_api::api::params::get_my_commands::GetMyCommands;
use telegram_bots_api::api::params::get_update::GetUpdate;
use telegram_bots_api::api::params::send_dice::SendDice;
use telegram_bots_api::api::params::set_my_commands::SetMyCommands;
use telegram_bots_api::api::requests::sync::Requests;
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
        let client = Sync::new(
            config.timeout,
            config.connect_timeout,
            config.url.as_str(),
            config.token.as_str(),
        );
        let user = User::from(client.get_me().unwrap());
        let webhook = Webhook::from(&config);

        Self::new(config, client, user, webhook)
    }
}

impl BotsApi {
    pub fn new(config: Config, client: Sync, user: User, webhook: Webhook) -> Self {
        Self {
            config,
            client,
            user,
            webhook,
        }
    }

    pub fn from_env() -> Self {
        Self::from(Config::new())
    }
}

impl Commander for BotsApi {
    fn commands(&self, params: (DeleteMyCommands, GetMyCommands, SetMyCommands)) {
        let (delete_params, _, set_params) = params;

        self.delete_commands(delete_params);
        self.set_commands(set_params);
    }

    fn delete_commands(&self, params: DeleteMyCommands) -> bool {
        self.client.delete_my_commands(&params).unwrap()
    }

    fn get_commands(&self, params: GetMyCommands) -> Vec<BotCommand> {
        self.client
            .get_my_commands(&params)
            .unwrap()
            .iter()
            .map(|inner| BotCommand::from(inner.to_owned()))
            .collect::<Vec<BotCommand>>()
    }

    fn set_commands(&self, params: SetMyCommands) -> bool {
        self.client.set_my_commands(&params).unwrap()
    }
}

impl Pooler for BotsApi {
    fn pooling(&self, callback: impl Fn(&BotsApi, Update)) {
        let mut update_offset = self.config.updates_offset;

        // self.delete_webhook();

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

            sleep(Duration::from_secs(self.config.pooling_timeout));
        }
    }
}

impl Sender for BotsApi {
    fn send_dice(&self, chat_id: i64) {
        self.client
            .send_dice(&SendDice {
                chat_id: ChatUId::from(chat_id),
                ..Default::default()
            })
            .unwrap();
    }
}

impl Webhooker for BotsApi {
    fn delete_webhook(&self) -> bool {
        let params = self.webhook.delete_params();

        self.client.delete_webhook(&params).unwrap()
    }

    fn get_webhook(&self) -> WebhookInfo {
        WebhookInfo::from(self.client.get_webhook_info().unwrap())
    }

    fn set_webhook(&self) -> bool {
        let params = self.webhook.set_params();

        self.client.set_webhook(&params).unwrap()
    }
}
