use crate::bots_api::BotsApi;

#[async_trait::async_trait]
impl HttpListen for BotsApi {
    async fn http_listen(&self) {
        todo!()
    }
}
