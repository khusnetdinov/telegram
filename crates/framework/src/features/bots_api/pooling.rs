use crate::bots_api::BotsApi;
use crate::enums::allowed_updates::AllowedUpdate;
use crate::structs::update::Update;
use crate::traits::bots_apis::pooling::Pooling;
use crate::traits::bots_apis::sealed::webhook::Webhook;
use crate::traits::storage::Storage;
use futures::Future;
use std::fmt::Debug;
use std::sync::Arc;
use telegram_bots_api::api::params::get_update::GetUpdate;
use telegram_bots_api::api::requests::r#async::Requests;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio::time::Duration;

#[async_trait::async_trait]
impl<STO, STA> Pooling<STO, STA> for BotsApi {
    async fn pooling<Callback, Fut>(
        &self,
        allowed_updates: Option<Vec<AllowedUpdate>>,
        storage: Arc<Mutex<STO>>,
        callback: Callback,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        Callback: Fn(BotsApi, Arc<Mutex<STO>>, Update) -> Fut + Send,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + 'static,
        STO: Storage<STA> + Debug + Send + Sync + 'async_trait,
        STA: Debug + Clone + 'async_trait,
    {
        let mut update_offset = self.config.updates_offset;
        let allowed_updates =
            allowed_updates.map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect());

        self.delete_webhook().await?;

        loop {
            let updates = self
                .client
                .get_updates(&GetUpdate {
                    offset: update_offset,
                    limit: self.config.updates_limit,
                    timeout: self.config.updates_timeout,
                    allowed_updates: allowed_updates.clone(),
                })
                .await
                .unwrap();

            for inner in updates.into_iter() {
                let offset = &inner.update_id + 1i64;

                callback(self.clone(), storage.clone(), Update::from(inner)).await?;

                update_offset = offset;
            }

            sleep(Duration::from_secs(self.config.pooling_timeout)).await;
        }
    }
}
