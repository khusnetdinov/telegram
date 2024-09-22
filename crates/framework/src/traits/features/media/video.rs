use crate::enums::chat_uid::ChatUId;
use crate::enums::file_input::FileInput;
use crate::structs::media::options::Options as MediaOptions;
use crate::structs::options::Options;
use crate::structs::updates::message::Message;

#[async_trait::async_trait]
pub trait Video {
    async fn send_video(
        &self,
        chat_id: ChatUId,
        file: FileInput,
        media_options: MediaOptions,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
