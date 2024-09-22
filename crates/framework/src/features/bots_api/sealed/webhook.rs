use crate::bots_api::BotsApi;
use crate::structs::webhook_info::WebhookInfo;
use crate::traits::bots_apis::sealed::webhook::Webhook;
use crate::traits::params::Params;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Webhook for BotsApi {
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
