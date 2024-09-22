use crate::enums::chat_uid::ChatUId;
use crate::structs::geo::location::Location as Send;
use crate::structs::geo::options::Options as GeoOptions;
use crate::structs::updates::message::Message;
#[async_trait::async_trait]
pub trait Location {
    async fn send_location(
        &self,
        chat_id: ChatUId,
        location: Send,
        options: GeoOptions,
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
