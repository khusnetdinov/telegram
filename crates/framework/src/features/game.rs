use crate::bots_api::BotsApi;
use crate::enums::message_result::MessageResult;
use crate::structs::games::game_high_score::GameHighScore;
use crate::structs::games::options::Options as GameOptions;
use crate::structs::options::Options;
use crate::structs::updates::message::Message;
use crate::traits::game::Game;
use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::params::get_game_high_scores::GetGameHighScores;
use telegram_bots_api::api::params::send_game::SendGame;
use telegram_bots_api::api::params::set_game_score::SetGameScore;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Game for BotsApi {
    async fn get_game_high_scores(
        &self,
        user_id: i64,
        chat_id: i64,
        game_options: Option<GameOptions>,
    ) -> Result<Vec<GameHighScore>, Box<dyn std::error::Error>> {
        let params = if let Some(options) = game_options {
            GetGameHighScores {
                user_id,
                chat_id: ChatUId::from(chat_id),
                message_id: options.message_id.map(|inner| inner.into()),
                inline_message_id: options.inline_message_id,
            }
        } else {
            GetGameHighScores {
                user_id,
                chat_id: ChatUId::from(chat_id),
                ..Default::default()
            }
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
        chat_id: i64,
        game_short_name: String,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let params = if let Some(options) = options {
            SendGame {
                game_short_name,
                chat_id: ChatUId::from(chat_id),
                business_connection_id: options.business_connection_id,
                disable_notification: options.disable_notification,
                protect_content: options.protect_content,
                message_effect_id: options.message_effect_id,
                message_thread_id: options.message_thread_id,
                reply_parameters: options.reply_parameters,
                reply_markup: options.reply_markup,
            }
        } else {
            SendGame {
                game_short_name,
                chat_id: ChatUId::from(chat_id),
                ..Default::default()
            }
        };

        Ok(self.client.send_game(&params).await?.into())
    }

    #[allow(clippy::redundant_closure)]
    async fn set_game_score(
        &self,
        user_id: i64,
        score: u64,
        game_options: Option<GameOptions>,
    ) -> Result<MessageResult, Box<dyn std::error::Error>> {
        let params = if let Some(options) = game_options {
            SetGameScore {
                user_id,
                score,
                force: options.force,
                disable_edit_message: options.disable_edit_message,
                chat_id: options.chat_id.map(|inner| ChatUId::from(inner)),
                message_id: options.message_id.map(|inner| inner.into()),
                inline_message_id: options.inline_message_id,
            }
        } else {
            SetGameScore {
                user_id,
                score,
                ..Default::default()
            }
        };

        Ok(self.client.set_game_score(&params).await?.into())
    }
}
