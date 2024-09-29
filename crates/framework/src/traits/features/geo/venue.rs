use crate::enums::chat_uid::ChatUId;
use crate::structs::geo::options::Options as GeoOptions;
use crate::structs::geo::venue::Venue as Send;
use crate::structs::message::Message;

#[async_trait::async_trait]
pub trait Venue {
    async fn send_venue(
        &self,
        chat_id: ChatUId,
        venue: Send,
        options: GeoOptions,
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
