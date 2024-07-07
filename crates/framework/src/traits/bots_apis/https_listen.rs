use crate::traits::webhook::Webhook;

#[async_trait::async_trait]
pub trait HttpsListen
where
    Self: Webhook
{
    fn https_listen(&self);
}
