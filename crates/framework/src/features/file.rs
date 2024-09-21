use crate::bots_api::BotsApi;
use crate::structs::file::File as Receive;
use crate::traits::features::file::File;
use telegram_bots_api::api::params::get_file::GetFile;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl File for BotsApi {
    async fn get_file(&self, file_id: String) -> Result<Receive, Box<dyn std::error::Error>> {
        let params = GetFile { file_id };

        Ok(self.client.get_file(&params).await?.into())
    }
}
