use crate::enums::chat_uid::ChatUId;
use crate::enums::emoji::Emoji;
use crate::structs::dice::options::Options as DiceOptions;
use crate::structs::updates::message::Message;

#[async_trait::async_trait]
pub trait Dice {
    async fn send_dice(
        &self,
        chat_id: ChatUId,
        emoji: Option<Emoji>,
        options: DiceOptions,
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
