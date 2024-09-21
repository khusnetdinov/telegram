use crate::enums::chat_action::ChatAction;
use crate::enums::chat_uid::ChatUId;
use crate::structs::options::Options;

#[async_trait::async_trait]
pub trait ChatActions {
    async fn send_chat_action(
        &self,
        chat_id: ChatUId,
        action: ChatAction,
        options: Option<Options>,
    ) -> Result<bool, Box<dyn std::error::Error>>;
}
