use crate::enums::chat_uid::ChatUId;
use crate::enums::message_result::MessageResult;
use crate::structs::games::game_high_score::GameHighScore;
use crate::structs::games::options::Options as GameOptions;
use crate::structs::messages::message_id::MessageId;
use crate::structs::updates::message::Message;

#[async_trait::async_trait]
pub trait Game {
    async fn get_game_high_scores(
        &self,
        user_id: i64,
        chat_id: ChatUId,
        message_id: Option<MessageId>,
        inline_message_id: Option<String>,
    ) -> Result<Vec<GameHighScore>, Box<dyn std::error::Error>>;

    async fn send_game(
        &self,
        chat_id: ChatUId,
        game_short_name: String,
        options: GameOptions,
    ) -> Result<Message, Box<dyn std::error::Error>>;

    async fn set_game_score(
        &self,
        score: u64,
        user_id: i64,
        chat_id: Option<ChatUId>,
        message_id: Option<MessageId>,
        inline_message_id: Option<String>,
        options: GameOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>>;
}
