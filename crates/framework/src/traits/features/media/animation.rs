use crate::enums::chat_uid::ChatUId;
use crate::enums::file_input::FileInput;
use crate::structs::media::options::Options as MediaOptions;
use crate::structs::updates::message::Message;

#[async_trait::async_trait]
pub trait Animation {
    async fn send_animation(
        &self,
        chat_id: ChatUId,
        file: FileInput,
        options: MediaOptions,
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
