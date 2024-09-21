use crate::enums::chat_uid::ChatUId;
use crate::enums::input_media::InputMedia;
use crate::feature::bots_api::Options;
use crate::structs::updates::message::Message;

#[async_trait::async_trait]
pub trait MediaGroup {
    async fn send_media_group(
        &self,
        chat_id: ChatUId,
        media: Vec<InputMedia>,
        option: Option<Options>,
    ) -> Result<Vec<Message>, Box<dyn std::error::Error>>;
}
