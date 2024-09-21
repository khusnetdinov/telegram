use crate::enums::chat_uid::ChatUId;
use crate::structs::options::Options;
use crate::structs::updates::message::Message;
use crate::structs::venue::Venue as Send;

#[async_trait::async_trait]
pub trait Venue {
    async fn send_venue(
        &self,
        chat_id: ChatUId,
        venue: Send,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
