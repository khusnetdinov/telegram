use crate::structs::options::Options;
use crate::structs::updates::message::Message;

#[async_trait::async_trait]
pub trait Game {
    async fn send_game(
        &self,
        chat_id: i64,
        game_short_name: String,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
