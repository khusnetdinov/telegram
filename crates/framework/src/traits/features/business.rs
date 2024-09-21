use crate::structs::business::business_connection::BusinessConnection;

#[async_trait::async_trait]
pub trait Business {
    async fn get_business_connection(
        &self,
        business_connection_id: String,
    ) -> Result<BusinessConnection, Box<dyn std::error::Error>>;
}
