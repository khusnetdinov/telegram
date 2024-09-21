use crate::traits::bots_apis::parametrized::webhook::Webhook;

#[async_trait::async_trait]
pub trait HttpsListen
where
    Self: Webhook,
{
    async fn https_listen(&self);
}
