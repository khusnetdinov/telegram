use crate::bots_api::BotsApi;
use crate::traits::bots_apis::https_listen::HttpsListen;

#[async_trait::async_trait]
impl HttpsListen for BotsApi {
    async fn https_listen(&self) {
        todo!()
    }
}
