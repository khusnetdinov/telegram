use crate::enums::chat_uid::ChatUId;
use crate::structs::options::Options;
use crate::structs::updates::incoming_messages::venue::Venue as Send;
use crate::structs::updates::message::Message;

#[async_trait::async_trait]
pub trait Venue {
    async fn send_venue(
        &self,
        chat_id: ChatUId,
        venue: Send,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
