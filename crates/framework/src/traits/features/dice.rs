use crate::enums::chat_uid::ChatUId;
use crate::enums::emoji::Emoji;
use crate::structs::options::Options;
use crate::structs::updates::message::Message;

#[async_trait::async_trait]
pub trait Dice {
    async fn send_dice(
        &self,
        chat_id: ChatUId,
        emoji: Option<Emoji>,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
