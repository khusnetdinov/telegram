use crate::traits::bots_apis::commands::Commands;
use crate::traits::webhook::Webhook;

#[async_trait::async_trait]
pub trait HttpsListen
where
    Self: Commands + Webhook,
{
    fn https_listen(&self);
}
