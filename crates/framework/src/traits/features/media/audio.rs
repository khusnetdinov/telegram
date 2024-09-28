use crate::enums::chat_uid::ChatUId;
use crate::enums::file_input::FileInput;
use crate::structs::media::options::Options as MediaOptions;
use crate::structs::updates::message::Message;

#[async_trait::async_trait]
pub trait Audio {
    async fn send_audio(
        &self,
        chat_id: ChatUId,
        file: FileInput,
        media_options: MediaOptions,
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
