use crate::enums::chat_action::ChatAction;
use crate::structs::options::Options;

#[async_trait::async_trait]
pub trait ChatActions {
    async fn send_chat_action(
        &self,
        chat_id: i64,
        action: ChatAction,
        options: Option<Options>,
    ) -> Result<bool, Box<dyn std::error::Error>>;
}
