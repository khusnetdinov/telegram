use crate::bots_api::BotsApi;
use crate::enums::allowed_updates::AllowedUpdate;
use crate::structs::update::Update;
use crate::traits::bots_apis::parametrized::webhook::Webhook;
use crate::traits::storage::Storage;
use futures::Future;
use std::fmt::Debug;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait Pooling<STO, STA>
where
    Self: Webhook,
{
    async fn pooling<Callback, Fut>(
        &self,
        allowed_updates: Option<Vec<AllowedUpdate>>,
        storage: Arc<STO>,
        callback: Callback,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        Callback: Fn(BotsApi, Arc<STO>, Update) -> Fut + std::marker::Send,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + 'static,
        STO: Storage<STA> + Debug + Send + Sync + 'async_trait,
        STA: Debug + Clone + 'async_trait;
}
