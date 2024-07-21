use crate::config::Config;
use crate::structs::user::User;
use crate::structs::webhook::Webhook;
use std::fmt::Debug;
use telegram_bots_api::api::requests::r#async::Requests;
use telegram_bots_api::clients::r#async::Async;

#[derive(Debug, Clone)]
pub struct BotsApi {
    pub config: Config,
    pub client: Async,
    pub user: User,
    pub webhook: Webhook,
}

impl BotsApi {
    pub async fn new(
        config: Config,
        client: Async,
        user: User,
        webhook: Webhook,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            config,
            client,
            user,
            webhook,
        })
    }

    pub async fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::new();
        let client = Async::new(
            config.timeout,
            config.connect_timeout,
            config.url.as_str(),
            config.token.as_str(),
        );

        let user = User::from(client.get_me().await?);
        let webhook = Webhook::from(&config);

        let bots_api = Self::new(config, client, user, webhook).await?;

        Ok(bots_api)
    }
}