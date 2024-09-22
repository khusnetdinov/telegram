use crate::enums::chat_uid::ChatUId;
use crate::enums::file_input::FileInput;
use crate::structs::media::options::Options as MediaOptions;
use crate::structs::updates::message::Message;

#[async_trait::async_trait]
pub trait Voice {
    async fn send_voice(
        &self,
        chat_id: ChatUId,
        file: FileInput,
        media_options: MediaOptions,
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
