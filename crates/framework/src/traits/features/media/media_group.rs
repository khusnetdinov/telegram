use crate::enums::chat_uid::ChatUId;
use crate::enums::input_media::InputMedia;
use crate::structs::media::options::Options as MediaOptions;
use crate::structs::message::Message;

#[async_trait::async_trait]
pub trait MediaGroup {
    async fn send_media_group(
        &self,
        chat_id: ChatUId,
        media: Vec<InputMedia>,
        options: MediaOptions,
    ) -> Result<Vec<Message>, Box<dyn std::error::Error>>;
}
