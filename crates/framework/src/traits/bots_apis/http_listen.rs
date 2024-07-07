use crate::traits::bots_apis::commands::Commands;
use crate::traits::webhook::Webhook;

#[async_trait::async_trait]
pub trait HttpListen
where
    Self: Commands + Webhook,
{
    fn http_listen(&self);
}
