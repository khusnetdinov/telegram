use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::enums::emoji::Emoji;
use crate::structs::options::Options;
use crate::structs::updates::message::Message;
use crate::traits::features::dice::Dice;
use telegram_bots_api::api::params::send_dice::SendDice;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Dice for BotsApi {
    async fn send_dice(
        &self,
        chat_id: ChatUId,
        emoji: Option<Emoji>,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let params = if let Some(options) = options {
            SendDice {
                chat_id: chat_id.into(),
                emoji: emoji.map(|emoji| emoji.into()),
                business_connection_id: options.business_connection_id,
                disable_notification: options.disable_notification,
                protect_content: options.protect_content,
                message_effect_id: options.message_effect_id,
                message_thread_id: options.message_thread_id,
                reply_parameters: options.reply_parameters,
                reply_markup: options.reply_markup,
            }
        } else {
            SendDice {
                chat_id: chat_id.into(),
                emoji: emoji.map(|emoji| emoji.into()),
                ..Default::default()
            }
        };

        Ok(self.client.send_dice(&params).await?.into())
    }
}
