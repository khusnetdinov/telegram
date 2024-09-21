use crate::enums::chat_uid::ChatUId;
use crate::structs::location::Location as Send;
use crate::structs::options::Options;
use crate::structs::updates::message::Message;
#[async_trait::async_trait]
pub trait Location {
    async fn send_location(
        &self,
        chat_id: ChatUId,
        location: Send,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
