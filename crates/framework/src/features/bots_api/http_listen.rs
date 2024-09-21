use crate::bots_api::BotsApi;
use crate::traits::bots_apis::http_listen::HttpListen;

#[async_trait::async_trait]
impl HttpListen for BotsApi {
    async fn http_listen(&self) {
        todo!()
    }
}
