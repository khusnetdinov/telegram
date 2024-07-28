use crate::structs::file::File as Receive;

#[async_trait::async_trait]
pub trait File {
    async fn get_file(&self, file_id: String) -> Result<Receive, Box<dyn std::error::Error>>;
}
