use crate::bots_api::BotsApi;
use crate::enums::input_media::InputMedia;
use crate::structs::options::Options;
use crate::structs::updates::message::Message;
use crate::traits::media_group::MediaGroup;
use std::error::Error;
use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::params::send_media_group::SendMediaGroup;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl MediaGroup for BotsApi {
    async fn send_media_group(
        &self,
        chat_id: i64,
        media: Vec<InputMedia>,
        option: Option<Options>,
    ) -> Result<Vec<Message>, Box<dyn Error>> {
        let params = if let Some(options) = option {
            SendMediaGroup {
                chat_id: ChatUId::from(chat_id),
                media: media.iter().map(|inner| inner.clone().into()).collect(),
                business_connection_id: options.business_connection_id,
                disable_notification: options.disable_notification,
                protect_content: options.protect_content,
                message_effect_id: options.message_effect_id,
                message_thread_id: options.message_thread_id,
                reply_parameters: options.reply_parameters,
            }
        } else {
            SendMediaGroup {
                chat_id: ChatUId::from(chat_id),
                media: media.iter().map(|inner| inner.clone().into()).collect(),
                ..Default::default()
            }
        };

        Ok(self
            .client
            .send_media_group(&params)
            .await?
            .iter()
            .map(|inner| inner.clone().into())
            .collect())
    }
}
