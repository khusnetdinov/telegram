#[async_trait::async_trait]
pub trait Poll {
    // async fn send_poll(
    //     &self,
    //     chat_id: i64,
    //     question: String,
    //     poll_options: Vec<InputPollOption>,
    //     options: Option<SendOptions>,
    // ) -> Result<Message, Box<dyn std::error::Error>>;
}
