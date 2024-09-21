use crate::traits::bots_apis::parametrized::webhook::Webhook;

#[async_trait::async_trait]
pub trait HttpListen
where
    Self: Webhook,
{
    async fn http_listen(&self);
}
