use crate::traits::webhook::Webhook;

#[async_trait::async_trait]
pub trait HttpListen
where
    Self: Webhook,
{
    fn http_listen(&self);
}
