use crate::enums::chat_uid::ChatUId;
use crate::enums::message_result::MessageResult;
use crate::structs::games::game_high_score::GameHighScore;
use crate::structs::games::options::Options as GameOptions;
use crate::structs::message::Message;
use crate::structs::messages::message_id::MessageId;

#[async_trait::async_trait]
pub trait Game {
    async fn get_game_high_scores(
        &self,
        user_id: i64,
        chat_id: ChatUId,
        message_id: MessageId,
    ) -> Result<Vec<GameHighScore>, Box<dyn std::error::Error>>;

    async fn get_game_high_scores_inline(
        &self,
        user_id: i64,
        inline_message_id: String,
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
        chat_id: ChatUId,
        message_id: MessageId,
        options: GameOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>>;

    async fn set_game_score_inline(
        &self,
        score: u64,
        user_id: i64,
        inline_message_id: String,
        options: GameOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>>;
}
