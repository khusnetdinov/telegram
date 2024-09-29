use crate::enums::chat_uid::ChatUId;
use crate::enums::file_input::FileInput;
use crate::structs::media::options::Options as MediaOptions;
use crate::structs::message::Message;

#[async_trait::async_trait]
pub trait Document {
    async fn send_document(
        &self,
        chat_id: ChatUId,
        file: FileInput,
        media_options: MediaOptions,
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
