use crate::bots_api::BotsApi;
use crate::structs::business::business_connection::BusinessConnection;
use crate::traits::business::Business;
use telegram_bots_api::api::params::get_business_connection::GetBusinessConnection;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Business for BotsApi {
    async fn get_business_connection(
        &self,
        business_connection_id: String,
    ) -> Result<BusinessConnection, Box<dyn std::error::Error>> {
        let params = GetBusinessConnection {
            business_connection_id,
        };

        Ok(self.client.get_business_connection(&params).await?.into())
    }
}
