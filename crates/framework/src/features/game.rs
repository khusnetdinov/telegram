use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::enums::message_result::MessageResult;
use crate::structs::games::game_high_score::GameHighScore;
use crate::structs::games::options::Options as GameOptions;
use crate::structs::message::Message;
use crate::structs::messages::message_id::MessageId;
use crate::traits::features::game::Game;
use telegram_bots_api::api::params::get_game_high_scores::GetGameHighScores;
use telegram_bots_api::api::params::send_game::SendGame;
use telegram_bots_api::api::params::set_game_score::SetGameScore;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Game for BotsApi {
    async fn get_game_high_scores(
        &self,
        user_id: i64,
        chat_id: ChatUId,
        message_id: MessageId,
    ) -> Result<Vec<GameHighScore>, Box<dyn std::error::Error>> {
        let params = GetGameHighScores {
            user_id,
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id.into()),
            inline_message_id: None,
        };

        Ok(self
            .client
            .get_game_high_scores(&params)
            .await?
            .iter()
            .map(|inner| inner.to_owned().into())
            .collect())
    }

    async fn get_game_high_scores_inline(
        &self,
        user_id: i64,
        inline_message_id: String,
    ) -> Result<Vec<GameHighScore>, Box<dyn std::error::Error>> {
        let params = GetGameHighScores {
            user_id,
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id),
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

    async fn set_game_score(
        &self,
        score: u64,
        user_id: i64,
        chat_id: ChatUId,
        message_id: MessageId,
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
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id.into()),
            inline_message_id: None,
            force,
            disable_edit_message,
        };

        Ok(self.client.set_game_score(&params).await?.into())
    }

    async fn set_game_score_inline(
        &self,
        score: u64,
        user_id: i64,
        inline_message_id: String,
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
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id),
            force,
            disable_edit_message,
        };

        Ok(self.client.set_game_score(&params).await?.into())
    }
}
