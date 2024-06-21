use crate::config::Config;
use crate::structs::bot_command::BotCommand;
// use crate::structs::update::Update;
use crate::structs::user::User;
use crate::structs::webhook::Webhook;
use crate::structs::webhook_info::WebhookInfo;
use crate::traits::bots_api::Commander;
use crate::traits::bots_api::Pooler;
// use crate::traits::bots_api::Sender;
use crate::traits::bots_api::Webhooker;
use crate::traits::params::Params;
use tokio::time::sleep;
use tokio::time::Duration;
// use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::params::delete_my_commands::DeleteMyCommands;
use telegram_bots_api::api::params::get_my_commands::GetMyCommands;
use telegram_bots_api::api::params::get_update::GetUpdate;
use telegram_bots_api::api::params::set_my_commands::SetMyCommands;
// use telegram_bots_api::api::params::send_dice::SendDice;
use telegram_bots_api::api::requests::r#async::Requests;
use telegram_bots_api::clients::r#async::Async;

#[derive(Debug)]
pub struct BotsApi {
    config: Config,
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

        // TODO: AuthorizationError
        let user = User::from(client.get_me().await?);
        let webhook = Webhook::from(&config);

        let bots_api = Self::new(config, client, user, webhook).await?;

        Ok(bots_api)
    }
}

#[async_trait::async_trait]
impl Commander for BotsApi {
    async fn commands(
        &self,
        params: (DeleteMyCommands, GetMyCommands, SetMyCommands),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (delete_params, _, set_params) = params;

        self.delete_commands(delete_params).await?;
        self.set_commands(set_params).await?;

        Ok(())
    }

    async fn delete_commands(
        &self,
        params: DeleteMyCommands,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.client.delete_my_commands(&params).await?)
    }

    async fn get_commands(
        &self,
        params: GetMyCommands,
    ) -> Result<Vec<BotCommand>, Box<dyn std::error::Error>> {
        Ok(self
            .client
            .get_my_commands(&params)
            .await?
            .iter()
            .map(|inner| BotCommand::from(inner.to_owned()))
            .collect::<Vec<BotCommand>>())
    }

    async fn set_commands(
        &self,
        params: SetMyCommands,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.client.set_my_commands(&params).await?)
    }
}

#[async_trait::async_trait]
impl Pooler for BotsApi {
    async fn pooling(
        &self,
        // callback: impl Fn(Update) + Send,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut update_offset = self.config.updates_offset;

        self.delete_webhook().await?;

        loop {
            let updates = self
                .client
                .get_updates(&GetUpdate {
                    offset: update_offset,
                    limit: self.config.updates_limit,
                    timeout: self.config.updates_timeout,
                    allowed_updates: None,
                })
                .await?;

            for inner in updates.into_iter() {
                let offset = &inner.update_id + 1i64;

                // callback(Update::from(inner));
                update_offset = offset;
            }

            sleep(Duration::from_secs(self.config.pooling_timeout)).await;
        }
    }
}

// impl Sender for BotsApi {
//     fn send_dice(&self, chat_id: i64) {
//         self.client
//             .send_dice(&SendDice {
//                 chat_id: ChatUId::from(chat_id),
//                 ..Default::default()
//             })
//             .unwrap();
//     }
// }

#[async_trait::async_trait]
impl Webhooker for BotsApi {
    async fn delete_webhook(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let params = self.webhook.delete_params();

        Ok(self.client.delete_webhook(&params).await?)
    }

    async fn get_webhook(&self) -> Result<WebhookInfo, Box<dyn std::error::Error>> {
        Ok(WebhookInfo::from(self.client.get_webhook_info().await?))
    }

    async fn set_webhook(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let params = self.webhook.set_params();

        Ok(self.client.set_webhook(&params).await?)
    }
}
