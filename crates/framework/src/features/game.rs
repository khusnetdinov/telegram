use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::enums::message_result::MessageResult;
use crate::structs::games::game_high_score::GameHighScore;
use crate::structs::games::options::Options as GameOptions;
use crate::structs::messages::message_id::MessageId;
use crate::structs::updates::message::Message;
use crate::traits::features::game::Game;
use telegram_bots_api::api::params::get_game_high_scores::GetGameHighScores;
use telegram_bots_api::api::params::send_game::SendGame;
use telegram_bots_api::api::params::set_game_score::SetGameScore;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Game for BotsApi {
    // TODO: Required if inline_message_id is not specified
    async fn get_game_high_scores(
        &self,
        user_id: i64,
        chat_id: ChatUId,
        message_id: Option<MessageId>,
        inline_message_id: Option<String>,
    ) -> Result<Vec<GameHighScore>, Box<dyn std::error::Error>> {
        let params = GetGameHighScores {
            user_id,
            chat_id: chat_id.into(),
            message_id: message_id.map(|inner| inner.into()),
            inline_message_id,
        };

        Ok(self
            .client
            .get_game_high_scores(&params)
            .await?
            .iter()
            .map(|inner| inner.to_owned().into())
            .collect())
    }

    async fn send_game(
        &self,
        chat_id: ChatUId,
        game_short_name: String,
        options: GameOptions,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let GameOptions {
            business_connection_id,
            disable_notification,
            protect_content,
            message_effect_id,
            message_thread_id,
            reply_parameters,
            reply_markup,
            ..
        } = options;

        let params = SendGame {
            game_short_name,
            chat_id: chat_id.into(),
            business_connection_id,
            disable_notification,
            protect_content,
            message_effect_id,
            message_thread_id,
            reply_parameters: reply_parameters.map(|inner| inner.into()),
            reply_markup: reply_markup.map(|inner| inner.into()),
        };

        Ok(self.client.send_game(&params).await?.into())
    }

    // TODO: Required if inline_message_id is not specified
    async fn set_game_score(
        &self,
        score: u64,
        user_id: i64,
        chat_id: Option<ChatUId>,
        message_id: Option<MessageId>,
        inline_message_id: Option<String>,
        options: GameOptions,
    ) -> Result<MessageResult, Box<dyn std::error::Error>> {
        let GameOptions {
            force,
            disable_edit_message,
            ..
        } = options;

        let params = SetGameScore {
            score,
            user_id,
            chat_id: chat_id.map(|inner| inner.into()),
            message_id: message_id.map(|inner| inner.into()),
            inline_message_id,
            force,
            disable_edit_message,
        };

        Ok(self.client.set_game_score(&params).await?.into())
    }
}
