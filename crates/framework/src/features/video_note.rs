use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::enums::file_input::FileInput;
use crate::feature::animation::MediaOptions;
use crate::feature::bots_api::Options;
use crate::structs::updates::message::Message;
use crate::traits::features::video_note::VideoNote;
use telegram_bots_api::api::params::send_video_note::SendVideoNote;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl VideoNote for BotsApi {
    async fn send_video_note(
        &self,
        chat_id: ChatUId,
        file: FileInput,
        media_options: MediaOptions,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let MediaOptions {
            duration,
            length,
            thumbnail,
            ..
        } = media_options;

        let params = if let Some(options) = options {
            SendVideoNote {
                chat_id: chat_id.into(),
                video_note: file.into(),
                duration,
                length,
                thumbnail,
                disable_notification: options.disable_notification,
                protect_content: options.protect_content,
                business_connection_id: options.business_connection_id,
                message_effect_id: options.message_effect_id,
                message_thread_id: options.message_thread_id,
                reply_parameters: options.reply_parameters,
                reply_markup: options.reply_markup,
            }
        } else {
            SendVideoNote {
                chat_id: chat_id.into(),
                video_note: file.into(),
                duration,
                length,
                thumbnail,
                ..Default::default()
            }
        };

        Ok(self.client.send_video_note(&params).await?.into())
    }
}
