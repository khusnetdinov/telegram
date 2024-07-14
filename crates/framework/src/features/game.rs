use crate::bots_api::BotsApi;
use crate::structs::options::Options;
use crate::structs::updates::message::Message;
use crate::traits::game::Game;
use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::params::send_game::SendGame;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Game for BotsApi {
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
}
