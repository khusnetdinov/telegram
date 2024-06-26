use crate::structs::webhook_info::WebhookInfo;

#[async_trait::async_trait]
pub trait Webhooker {
    async fn delete_webhook(&self) -> Result<bool, Box<dyn std::error::Error>>;

    async fn get_webhook(&self) -> Result<WebhookInfo, Box<dyn std::error::Error>>;

    async fn set_webhook(&self) -> Result<bool, Box<dyn std::error::Error>>;
}
