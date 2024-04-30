use crate::config::Config;
use crate::structs::webhook::Webhook;
use std::fmt::Debug;
use std::thread::sleep;
use std::time::Duration;
use telegram_bots_api::api::enums::menu_buttons::MenuButtons;
use telegram_bots_api::api::params::delete_webhook::DeleteWebhook;
use telegram_bots_api::api::params::get_update::GetUpdate;
use telegram_bots_api::api::params::set_webhook::SetWebhook;
use telegram_bots_api::api::requests::sync::Requests;
use telegram_bots_api::api::structs::bot_command::BotCommand;
use telegram_bots_api::api::structs::update::Update;
use telegram_bots_api::api::structs::user::User;
use telegram_bots_api::clients::sync::Sync;

#[derive(Debug)]
pub struct BotsApi {
    /// Bots commands
    bot_commands: Option<Vec<BotCommand>>,
    /// MenuButton
    menu_button: Option<MenuButtons>,
    /// Webhook: present if listen http(s)
    webhook: Webhook,
    /// Reqwest client
    client: Sync,
    /// BotsApi Configuration
    config: telegram_bots_api::config::Config,
    /// BotsApi User: present if successfully authenticated
    user: User,
}

impl From<Config> for BotsApi {
    fn from(config: Config) -> Self {
        let client = Sync::from(config.inner);
        let webhook = Webhook::from(&client.config);
        let config = client.config.clone();
        let user = client.get_me().unwrap();

        Self {
            bot_commands: None,
            menu_button: None,
            webhook,
            config,
            client,
            user,
        }
    }
}

impl BotsApi {
    pub fn new() -> Self {
        let client = Sync::new();
        let webhook = Webhook::from(&client.config);
        let config = client.config.clone();
        let user = client.get_me().unwrap();

        Self {
            bot_commands: None,
            menu_button: None,
            webhook,
            config,
            client,
            user,
        }
    }

    pub fn bot_commands(&self, bot_commands: Vec<BotCommand>) {
        println!("{:?}", bot_commands);
    }

    pub fn menu_button(&self, _menu_button: MenuButtons) {
        todo!()
    }

    /// Pooling updates for telegram bots api. Pass callback `fn(updates: Vec<Update>)`
    pub fn pooling(&self, drop_pending_updates: bool, callback: fn(updates: Update)) {
        // before_action: webhook:begin
        self.client
            .delete_webhook(&DeleteWebhook {
                drop_pending_updates: Some(drop_pending_updates),
            })
            .unwrap();
        // before_action: webhook:end

        // before_action: Commands, MenuButton

        loop {
            let params = GetUpdate {
                offset: self.config.updates_offset,
                limit: self.config.updates_limit,
                timeout: self.config.updates_timeout,
                allowed_updates: None,
            };
            let updates = self.client.get_updates(&params).unwrap();

            for update in updates.into_iter() {
                callback(update);
            }

            sleep(Duration::from_secs(1));
        }
    }

    pub fn listen_http(&self) {
        // before_action: webhook:begin
        let webhook = Webhook::from(self.client.get_webhook_info().unwrap());

        if *self.webhook != *webhook {
            self.client
                .set_webhook(&SetWebhook {
                    ..Default::default()
                })
                .unwrap();
        }
        // before_action: webhook:end

        // before_action: Commands, MenuButton

        loop {
            println!("Http Tick!");
            sleep(Duration::from_secs(1));
        }
    }

    pub fn listen_https(&self) {
        // before_action: webhook:begin
        let webhook = Webhook::from(self.client.get_webhook_info().unwrap());

        if *self.webhook != *webhook {
            self.client
                .set_webhook(&SetWebhook {
                    ..Default::default()
                })
                .unwrap();
        }
        // before_action: webhook:end

        // before_action: Commands, MenuButton

        loop {
            println!("Https Tick!");
            sleep(Duration::from_secs(1));
        }
    }
}
