use crate::bots_api::BotsApi;
use crate::structs::update::Update;
use crate::traits::storage::Storage;
use crate::traits::webhook::Webhook;
use futures::Future;
use std::fmt::Debug;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait Pooling<STO, STA> where Self: Webhook {
    async fn pooling<Callback, Fut>(
        &self,
        storage: Arc<STO>,
        callback: Callback,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        Callback: Fn(BotsApi, Arc<STO>, Update) -> Fut + std::marker::Send,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + 'static,
        STO: Storage<STA> + Debug + Send + Sync + 'async_trait,
        STA: Debug + Clone + 'async_trait;
}
