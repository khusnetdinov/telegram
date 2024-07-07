#[async_trait::async_trait]
pub trait Message {
    // async fn send_message(
    //     &self,
    //     chat_id: i64,
    //     text: String,
    //     options: Option<SendOptions>,
    // ) -> Result<Message, Box<dyn std::error::Error>>;
}
