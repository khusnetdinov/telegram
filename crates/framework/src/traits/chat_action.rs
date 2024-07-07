#[async_trait::async_trait]
pub trait ChatAction {
    // async fn send_chat_action(
    //     &self,
    //     chat_id: i64,
    //     action: ChatAction,
    //     options: Option<SendOptions>,
    // ) -> Result<bool, Box<dyn std::error::Error>>;
}
